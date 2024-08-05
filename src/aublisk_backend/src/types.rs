use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub fee : u64
}

#[derive(CandidType, Deserialize, Clone, Default)]
pub struct DashboardMetric {
    number_of_project: usize,
    total_amount_spent: usize,
    total_amount_received: usize
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UpdatableProject {
    title : Option<String>,
    description : Option<String>,
    amount_funded: Option<u64>,
    skill_sets: Option<Vec<String>>,
    icrc_token_address: Option<String>,
    milestones: Option<Vec<Milestone>>,
}

impl UpdatableProject {
    pub fn update(self, r_proj : Project, status : ProjectStatus) -> Result<Project, String> {
        let mut proj = Project {
            ..r_proj
        };
        if self.title.is_some() {
            proj.title = self.title.unwrap();
        }

        if self.description.is_some() {
            proj.description = self.description.unwrap()
        }

        if self.skill_sets.is_some() {

            match status {
                ProjectStatus::INPROGRESS | ProjectStatus::PUBLISHED | ProjectStatus::COMPLETED => {
                    return Err("You cannot change skillset at current project state".to_string());
                },

                _ => {
                    proj.skill_sets = self.skill_sets.unwrap();
                }
            }
           
        }

        if self.milestones.is_some() {

            match status {
                ProjectStatus::INPROGRESS | ProjectStatus::PUBLISHED | ProjectStatus::COMPLETED => {
                    return Err("You cannot change milestones at current project state".to_string());
                },

                _ => {
                    proj.milestones = self.milestones.unwrap();
                }
            }
           
        }

        if self.icrc_token_address.is_some() {
            if let ProjectStatus::CREATED = status {
                proj.icrc_token_address = self.icrc_token_address.unwrap();
            } else {
                return Err("You cannot change skillset of Published Project".to_string());
            }
        }

        if self.amount_funded.is_some() {
            if let ProjectStatus::CREATED = status {
                proj.amount_funded = self.amount_funded.unwrap();
            } else {
                return Err("You cannot change amount of funded project".to_string());
            }
        }

        Ok(proj)
    }
}

// Define the models
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Project {
    pub owner: Principal,
    pub milestones: Vec<Milestone>,
    pub amount_funded: u64,
    hash_id: String,
    title: String,
    description: String,
    pub skill_sets: Vec<String>,
    pub icrc_token_address: String,
    pub timestamp : u64
    // pub proposals: Vec<Proposal>,
}

impl Project {
    // pub fn new() -> Self {
    //     Project { owner: Principal::anonymous(), milestones: vec![], amount_funded: 0, hash_id: "".to_string(), title: "".to_string(), description: "".to_string(), skill_sets: vec![], icrc_token_address: "".to_string(),  }
    // }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum MilestoneStatus {
    Undone,
    InProgress,
    Done
}
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Milestone {
    pub member: Principal,
    pub task: String,
    pub payment_amount: u64,
    pub milestone_status : MilestoneStatus
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Proposal {
    pub sender: Principal,
    pub content: String,
    pub attachments: Vec<Attachment>,
    pub timestamp : u64
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct Attachment(Vec<u8>);

#[derive(CandidType, Deserialize, Clone)]
pub struct Member {
    pub name: String,
    pub description: String,
    pub principal: Principal,
    pub skill_sets: Vec<String>,
    pub resume: Vec<u8>,
    pub photo_url: String,
    pub nick_name: String
}

#[derive(CandidType, Deserialize, Clone, PartialEq)]
pub enum MemberStatus {
    AVAILABLE,
    UNAVAILABLE,
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq, Copy)]
pub enum ProjectStatus {
    CREATED = 0,
    FUNDED = 1,
    PUBLISHED = 2,
    INPROGRESS = 3,
    COMPLETED = 4
}

impl PartialOrd for ProjectStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProjectStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

#[derive(Default)]
pub struct AppState {
    pub projects : HashMap<String, Project>,
    pub members : HashMap<Principal, Member>,
    pub skillsets_supply : HashMap<String, u64>,
    pub skillsets_demand : HashMap<String, u64>,
    pub project_status: HashMap<String, ProjectStatus>,
    pub member_status : HashMap<Principal, MemberStatus>,
    pub project_manager: HashMap<Principal, HashSet<String>>,
    pub user_dmetrics : HashMap<Principal, DashboardMetric>,
    // bookkeeping purposses
    pub token_used : HashSet<String>,
    //member for set of skills
    pub skillset_by_member: HashMap<Principal, HashSet<String>>,
    // id for proposal
    pub project_proposals : HashMap<String, Vec<Proposal>>,
    //id for chat link
    pub project_contacts : HashMap<String, String>,

    // id for set of workers
    pub project_workers : HashMap<String, HashSet<Principal>>
}

impl AppState {
    pub fn create_project(&mut self, mut project : Project, hash_id : String, caller : Principal) -> Result<String, String> {

        if self.projects.contains_key(&hash_id) {
            return Err("Generated Hash Id collided".to_string())
        }
        project.owner = caller;
        project.hash_id = hash_id.clone();
        project.milestones = vec![];

        self.projects.insert(hash_id.clone(), project);

        self.project_status.insert(hash_id.clone(), ProjectStatus::CREATED);

        //TODO: Add project skillset to number of needed skills

        let set = self.project_manager.entry(caller).or_insert(HashSet::new());
        set.insert(hash_id.clone());
        Ok(hash_id)
    }

    pub fn get_user_projects(&self, caller : Principal) -> Vec<(Project, ProjectStatus)> {
        let set_opt = self.project_manager.get(&caller);
        let mut project = vec![];
        if set_opt.is_none() {
            return project
        }

        let set = set_opt.unwrap();
        for key in set {
            let project_opt = self.projects.get(key);

            if project_opt.is_none() {
                continue;
            }

            let project_status_opt = self.project_status.get(key);

            if project_status_opt.is_none() {
                continue;
            }

            project.push((project_opt.cloned().unwrap(), project_status_opt.cloned().unwrap()))
        }

        project
    }

    pub fn get_latest_projects(&self) -> Vec<Project> {
        let mut key_vec = vec![];
        let mut proj_vec = vec![];
        for (k, v) in self.project_status.iter() {
            if let ProjectStatus::PUBLISHED = v {
                key_vec.push(k)
            }
        }

        for key in key_vec {
            let project = self.projects.get(key);

            if project.is_none() {
                continue;
            }

            proj_vec.push(project.cloned().unwrap());
        }

        proj_vec
    }
}
#[derive(Debug, Clone)]
pub struct ExpiryData {
    pub owner : Principal,
    pub skill : String
}
#[derive(Debug, Clone)]
pub struct ExpiryResource<T> {
    pub expiration : u64, // millisecs
    pub data : Option<T>
}

impl <T>Ord for ExpiryResource<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.expiration.cmp(&other.expiration)
    }
}

impl<T> PartialEq for ExpiryResource<T> {
    fn eq(&self, other: &Self) -> bool {
        self.expiration == other.expiration
    }
}

impl<T> Eq for ExpiryResource<T> {}

impl<T> PartialOrd for ExpiryResource<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

