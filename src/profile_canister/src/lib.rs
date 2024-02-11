use candid::Nat;
use ic_cdk::{
    api::{ time}, caller, query, update
};
use types::{Profile, UpdateProfile};

mod types {
    use candid::CandidType;
    use serde::Deserialize;

    #[derive(CandidType, Deserialize, Clone)]
    pub struct Profile {
        // this username is unique
        pub username: String,
        pub bio: String,
        pub display_name: String,
        pub photo : Option<Vec<u8>>,
        pub verified : bool
        // followers: HashSet<String>,
        // following: HashSet<String>,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct UpdateProfile {
        pub bio : Option<String>,
        pub display_name: Option<String>,
    }

    impl UpdateProfile {
        pub fn mutate_profile(&self, profile : &mut Profile) {
            if self.bio.is_some() {
                profile.bio = self.bio.clone().unwrap();
            }

            if self.display_name.is_some() {
                profile.display_name = self.display_name.clone().unwrap();
            }
        }
    }
}

mod profile {
    use std::{
        cell::RefCell,
        collections::{HashMap, HashSet},
    };

    use candid::{Nat, Principal};
    use ic_cdk::caller;

    thread_local!(
        pub static USER_REGISTER: RefCell<Register> = RefCell::new(Register::default());
    );
    use crate::types::Profile;

    pub fn with<T, F: FnOnce(&Register) -> T>(f: F) -> T {
        USER_REGISTER.with(|ledger| f(&ledger.borrow()))
    }

    pub fn with_mut<T, F: FnOnce(&mut Register) -> T>(f: F) -> T {
        USER_REGISTER.with(|ledger| f(&mut ledger.borrow_mut()))
    }

    #[derive(Default)]
    pub struct Register {
        pub user_followers: HashMap<Principal, HashSet<Principal>>,
        pub user_followings: HashMap<Principal, HashSet<Principal>>,
        pub profiles: HashMap<Principal, Profile>,
        pub usernames: HashMap<String, Principal>,
    }

    impl Register {
        pub fn follower_count(&self) -> Nat {
            self.user_followers
                .get(&caller())
                .map_or(0u32.into(), |v| v.len().into())
        }

        pub fn following_count(&self) -> Nat {
            self.user_followings
                .get(&caller())
                .map_or(0u32.into(), |v| v.len().into())
        }

        pub fn check_username_existence(&self, username : &String) -> bool {
            self.usernames.contains_key(username)
        }

        pub fn register_profile(&mut self, profile : Profile) {
            self.usernames.insert(profile.username.clone(), caller());
            self.profiles.insert(caller(), profile);
        }

        pub fn follow_user(&mut self, user : Principal) {
            let followers_set = self.user_followings.get_mut(&caller()).unwrap();
            followers_set.insert(user);

            let user_followers = self.user_followers.get_mut(&user).unwrap();
            user_followers.insert(caller());
        }

        pub fn unfollow_user(&mut self, user : Principal) {
            let followers_set = self.user_followings.get_mut(&caller()).unwrap();
            followers_set.remove(&user);

            let user_followers = self.user_followers.get_mut(&user).unwrap();
            user_followers.remove(&caller());
        }



    }

   
}
// This Function get profile if it exists or create if it doesn't
#[update]
fn get_profile() -> Result<Profile, ()> {
    profile::with_mut(|register| {
        let optional_rslt = register.profiles.get(&caller());
        if optional_rslt.is_none() {
            let n_username = format!("User{}", time());
            let new_profile = Profile {
                bio: "".to_string(),
                display_name: n_username.clone(),
                verified: false,
                username: n_username.to_lowercase(),
                photo: None
            };
            if register.check_username_existence(&new_profile.username) {
                return get_profile();
            } else {
                register.register_profile(new_profile.clone())
            }

            Ok(new_profile)
        } else {
            let profile = optional_rslt.cloned().unwrap();
            Ok(profile)
        }
    })
}

#[update]
fn update_profile(p : UpdateProfile) -> Result<Profile, ()> {
    profile::with_mut(|register| {
        let prev_profile = register.profiles.get_mut(&caller()).unwrap();
        p.mutate_profile(prev_profile);
        Ok(prev_profile.clone())
    })
}

#[update]
fn follow_user(username : String) -> Result<Nat, ()> {
    profile::with_mut(|register| {
        let user_principal = register.usernames.get(&username).cloned().unwrap();
        register.follow_user(user_principal);
        Ok(200u32.into())
    })
}

#[update]
fn unfollow_user(username : String) -> Result<Nat, ()> {
    profile::with_mut(|register| {
        let user_principal = register.usernames.get(&username).cloned().unwrap();
        register.unfollow_user(user_principal);
        Ok(200u32.into())
    })
}

#[query]
fn get_follower_count() -> Result<Nat, ()> {
    profile::with_mut(|register|{
        Ok(register.follower_count())
    })
}

#[query]
fn get_following_count() -> Result<Nat, ()> {
    profile::with_mut(|register|{
        Ok(register.following_count())
    })
}

#[query]
fn get_followers() -> Result<Vec<Profile>, ()> {
    profile::with(|register| {
        let followers_set = register.user_followers.get(&caller()).unwrap();
        let mut profile_vector : Vec<Profile> = vec![];
        for follower in followers_set {
            let optional_rslt = register.profiles.get(follower);
            if optional_rslt.is_none() {
                continue;
            }

            let profile = optional_rslt.cloned().unwrap();
            profile_vector.push(profile);
        }

        Ok(profile_vector)
    })
}

#[query]
fn get_followings() -> Result<Vec<Profile>, ()> {
    profile::with(|register| {
        let followers_set = register.user_followings.get(&caller()).unwrap();
        let mut profile_vector : Vec<Profile> = vec![];
        for follower in followers_set {
            let optional_rslt = register.profiles.get(follower);
            if optional_rslt.is_none() {
                continue;
            }

            let profile = optional_rslt.cloned().unwrap();
            profile_vector.push(profile);
        }

        Ok(profile_vector)
    })
}

#[query]
fn export_candid() -> String {
    ic_cdk::export_candid!();
    __export_service()
}