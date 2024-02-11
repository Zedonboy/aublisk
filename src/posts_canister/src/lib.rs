use app_core::{config::NON_PREMIUM_POST_LIMIT, is_premium_user};
use candid::Nat;
use ic_cdk::{caller, query, update};
use post::{CreatePostSchema, Post, ServiceError};

mod post {
    use std::{
        cell::RefCell,
        collections::{HashMap, HashSet},
        default,
        fs::Metadata,
    };

    use candid::{CandidType, Principal};
    use regex::Regex;
    use serde::Deserialize;

    #[derive(CandidType)]
    pub enum ServiceError {
        PostTooLong,
        PostNotFound
    }

    #[derive(CandidType, Deserialize, Clone, Default)]
    pub enum PostType {
        REPOST,
        COMMENT,
        REQUOTE,
        #[default]
        NORMAL_POST,
    }

    #[derive(Default, CandidType, Clone)]
    pub struct PostMetadata {
        hash_tags: HashSet<String>,
        mentions: HashSet<String>,
        photos: Vec<String>,
        links: Vec<String>,
    }

    #[derive(Default, CandidType, Clone)]
    pub struct Post {
        pub id: String,
        pub content: String,
        pub creator: String,
        time: u64,
        metadata: Option<PostMetadata>,
        post_type: PostType,
    }

    impl Post {
        pub fn mutate_from_schema(&mut self, schema: CreatePostSchema) {
            if schema.content.is_some() {
                self.content = schema.content.unwrap()
            }

            if schema.photos.is_some() {
                let mut metadata = PostMetadata::default();
                metadata.photos = schema.photos.unwrap();
                self.metadata = Some(metadata)
            }

            self.post_type = schema.post_type
        }
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct CreatePostSchema {
        content: Option<String>,
        photos: Option<Vec<String>>,
        post_type: PostType,
    }

    thread_local!(
        pub static SERVICE: RefCell<Service> = RefCell::new(Service::default());
    );

    pub fn with<T, F: FnOnce(&Service) -> T>(f: F) -> T {
        SERVICE.with(|ledger| f(&ledger.borrow()))
    }

    pub fn with_mut<T, F: FnOnce(&mut Service) -> T>(f: F) -> T {
        SERVICE.with(|ledger| f(&mut ledger.borrow_mut()))
    }

    #[derive(Default)]
    pub struct Service {
        pub posts: HashMap<String, Post>,
    }

    impl Service {
        pub fn parse_content(input: &String) -> PostMetadata {
            let hashtag_re = Regex::new(r"#\w+").unwrap();
            let mention_re = Regex::new(r"@\w+").unwrap();
            // This URL regex is very simplistic and might not match all valid URLs accurately.
            let url_re = Regex::new(r"http[s]?://\S+").unwrap();

            let hashtags = hashtag_re
                .find_iter(input)
                .map(|mat| mat.as_str().to_string())
                .collect();

            let mentions = mention_re
                .find_iter(input)
                .map(|mat| mat.as_str().to_string())
                .collect();

            let links = url_re
                .find_iter(input)
                .map(|mat| mat.as_str().to_string())
                .collect();

            let mut metadata = PostMetadata::default();

            metadata.hash_tags = hashtags;
            metadata.mentions = mentions;
            metadata.links = links;

            return metadata;
        }
    }
}

#[update]
async fn create_post(schema: CreatePostSchema) -> Result<Nat, ServiceError> {
    let (proposed_id_vec,) = ic_cdk::api::management_canister::main::raw_rand().await.unwrap();
    let proposed_id = hex::encode(proposed_id_vec);
    let person = caller();
    let user_premium:Result<Nat, ()> = Ok(Nat::from(200u32)); //is_premium_user(person).await;
    post::with_mut(|service| {
        if service.posts.contains_key(&proposed_id) {
            ic_cdk::trap("The auto generated post id collided");
        }

        let mut post = Post::default();
        post.id = proposed_id;
        post.creator = person.to_text();
        post.mutate_from_schema(schema);

        if post.content.len() >= NON_PREMIUM_POST_LIMIT && !user_premium.is_ok() {
            return Err(ServiceError::PostTooLong);
        }

        service.posts.insert(post.id.clone(), post);
        Ok(Nat::from(200u32))
    })
}

#[query]
async fn get_post(id: String) -> Result<Post, ServiceError> {
    post::with(|service| {
        if service.posts.contains_key(&id) {
            let post = service.posts.get(&id).cloned().unwrap();
            Ok(post)
        } else {
            Err(ServiceError::PostNotFound)
        }
    })
}

#[query]
fn export_candid() -> String {
    ic_cdk::export_candid!();
    __export_service()
}
