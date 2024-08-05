use crate::stake::Neuron;
use candid::{candid_method, Nat, Principal};
use ic_cdk::{
    api::{self, call::call_with_payment, management_canister::main::raw_rand, time},
    call, caller, id, init, query, trap, update,
};
use icrc_ledger_types::{
    icrc::generic_metadata_value::MetadataValue,
    icrc1::{
        account::Account,
        transfer::{TransferArg, TransferError},
    },
};
use redblack::RedBlackTree;
use stake::{
    is_user_staked, NATIVE_TOKEN_DECIMAL, NATIVE_TOKEN_ID, NATIVE_TOKEN_SYMBOL,
    NATIVE_TOKEN_TRANSFER_FEE,
};
use std::{cell::RefCell, collections::HashSet, str::FromStr, time::Duration};
use types::{
    AppState, DashboardMetric, ExpiryData, ExpiryResource, Member, MemberStatus, Milestone,
    MilestoneStatus, Project, ProjectStatus, Proposal, UpdatableProject,
};
use utils::{
    check_icrc_balance, days_to_milliseconds, disburse_payment, get_amount_of_token_per_dollar,
    principal_to_subaccount, process_metadata, transfer_to_account, transfer_to_purse,
};
mod redblack;
mod stake;
mod types;
mod utils;
// Global state
thread_local! {
    static STATE : RefCell<AppState> = RefCell::new(AppState::default());
    static SORTED_DATA: std::cell::RefCell<RedBlackTree<ExpiryResource<ExpiryData>>> = std::cell::RefCell::new(RedBlackTree::new());
}

#[init]
fn init() {
    ic_cdk_timers::set_timer_interval(Duration::from_secs(24 * 3600), check_expired_skillsets);
    ic_cdk::println!("Resource expiration monitor initialized");
}

fn check_expired_skillsets() {
    SORTED_DATA.with_borrow_mut(|rb| {
        let expiry = ExpiryResource {
            expiration: (api::time() / 1_000_000),
            data: None,
        };
        let list = rb.remove_less_than_or_equal(&expiry);

        for resource in list {
            if resource.data.is_none() {
                continue;
            }

            let data = resource.data.unwrap();
            let state_rslt = STATE.with_borrow_mut(|state| -> Result<(), String> {
                let set = state
                    .skillset_by_member
                    .get_mut(&data.owner)
                    .ok_or("No user found with skillsets".to_string())?;
                set.remove(&data.skill);

                let count = state.skillsets_supply.entry(data.skill).or_insert(1);
                *count -= 1;
                Ok(())
            });

            if state_rslt.is_err() {
                ic_cdk::println!("Error trying to access user with skillset");
            }
        }
    });
}

// API 1: CRUD Project Model
#[update(guard = "not_anonymous")]
#[candid_method(update)]
async fn create_project(mut project: Project, project_contact: String) -> Result<String, String> {
    let (hash,) = raw_rand().await.unwrap();
    let hash_id = hex::encode(hash);
    STATE.with_borrow_mut(|appstate| {
        project.timestamp = time();
        let r = appstate.create_project(project, hash_id.clone(), caller());
        appstate.project_contacts.insert(hash_id, project_contact);
        r
    })
}

#[query]
#[candid_method(query)]
async fn get_project_address(hash_id: String) -> String {
    let bytes = hex::decode(hash_id).unwrap();
    if bytes.len() != 32 {
        trap("hash id is not 32 bytes");
    }
    let account = Account {
        owner: id(),
        subaccount: Some(bytes.try_into().unwrap()),
    };

    account.to_string()
}

#[query(guard = "not_anonymous")]
#[candid_method(query)]
async fn get_user_projects() -> Vec<(Project, ProjectStatus)> {
    STATE.with_borrow(|state| state.get_user_projects(caller()))
}

#[query]
#[candid_method(query)]
async fn get_latest_projects() -> Vec<Project> {
    STATE.with_borrow(|state| state.get_latest_projects())
}

#[update]
#[candid_method(query)]
async fn notify_project_fund(project_id: String) -> Result<(), String> {
    is_project_owner(&project_id, &caller())?;
    let (canister_id, fund_amt) =
        STATE.with_borrow(|state| -> Result<(Principal, u64), String> {
            let project = state
                .projects
                .get(&project_id)
                .ok_or("Project not found".to_string())?;
            let canister_id = Principal::from_str(&project.icrc_token_address).unwrap();
            Ok((canister_id, project.amount_funded))
        })?;

    let bytes = hex::decode(project_id.clone()).unwrap();

    let account = Account {
        owner: id(),
        subaccount: Some(bytes.try_into().unwrap()),
    };

    let (amount,): (Nat,) = ic_cdk::call(canister_id, "icrc1_balance_of", (account,))
        .await
        .unwrap();

    if amount >= fund_amt {
        STATE.with_borrow_mut(|state| {
            let proj = state.projects.get_mut(&project_id).unwrap();
            let actual_fund : u64 = amount.0.try_into().unwrap();
            proj.amount_funded = actual_fund;
            state
                .project_status
                .insert(project_id, ProjectStatus::FUNDED)
        });
        return Ok(());
    } else {
        return Err("Amount in project wallet is less than stipulated amount".to_string());
    }
}

#[update(guard = "not_anonymous")]
#[candid_method(update)]
async fn publish_project(hash_id: String) -> Result<(), String> {
    is_project_owner(&hash_id, &caller())?;

    // the following codes tries to take a dollar worth of token as service charge.

    let icrc_id = STATE.with_borrow(|state| -> Result<String, String> {
        let project = state.projects.get(&hash_id).ok_or("Project Not Found")?;

        Ok(project.icrc_token_address.clone())
    })?;

    let icrc_id = Principal::from_text(icrc_id).unwrap();
    let (symbol,): (String,) = call(icrc_id, "icrc1_symbol", ()).await.unwrap();
    let (decimal,): (u8,) = call(icrc_id, "icrc1_decimal", ()).await.unwrap();

    let tokens_per_dollar = get_amount_of_token_per_dollar(symbol, decimal as usize).await?;

    let subaccount_arr: [u8; 32] = hex::decode(hash_id.clone()).unwrap().try_into().unwrap();
    transfer_to_purse(icrc_id, tokens_per_dollar, Some(subaccount_arr)).await?;

    STATE.with_borrow_mut(|state| {
        let project = state.projects.get(&hash_id).ok_or("Project Not Found")?;
        let status = state
            .project_status
            .get(&hash_id)
            .ok_or("Could not find previous project status".to_string())?;

        if let ProjectStatus::FUNDED = status {
            state
                .project_status
                .insert(hash_id, ProjectStatus::PUBLISHED);

            for skill in &project.skill_sets {
                let prev = state.skillsets_demand.entry(skill.clone()).or_insert(0);
                *prev += 1
            }
            return Ok(());
        } else {
            return Err("Could not publish because this project is not funded".to_string());
        };

    })
}

// this function is called by owner of project
#[query]
#[candid_method(query)]
fn read_project_and_status(hash_id: String) -> Option<(Project, ProjectStatus)> {
    STATE.with_borrow(|state| {
        let project_opt = state.projects.get(&hash_id).cloned();
        let status_opt = state.project_status.get(&hash_id).cloned();
        if status_opt.is_none() || project_opt.is_none() {
            return None
        } else {
            return Some((project_opt.unwrap(), status_opt.unwrap()));
        }
    })
}

#[update]
fn update_project(hash_id: String, updated_project: UpdatableProject) -> Result<(), String> {
    let caller = ic_cdk::caller();
    is_project_owner(&hash_id, &caller)?;
    STATE.with_borrow_mut(|state| -> Result<(), String> {
        let project = state
            .projects
            .get(&hash_id)
            .cloned()
            .ok_or("Project Not Found".to_string())?;
        let project_status = state.project_status.get(&hash_id).cloned().unwrap();

        let new_proj = updated_project.update(project, project_status)?;
        state.projects.insert(hash_id, new_proj);
        Ok(())
    })?;
    Ok(())
}

// #[update]
// fn delete_project(hash_id: String) -> Result<(), String> {
//     let caller = ic_cdk::caller();
//     if !is_project_owner(&hash_id, &caller) {
//         return Err("Only the project owner can delete the project".to_string());
//     }
//     PROJECTS.with(|projects| {
//         projects.borrow_mut().remove(&hash_id);
//     });
//     Ok(())
// }

// API 2: Add milestones to Project Model
#[update]
fn add_milestones(hash_id: String, milestones: Vec<Milestone>) -> Result<(), String> {
    let caller = ic_cdk::caller();
    is_project_owner(&hash_id, &caller)?;
    STATE.with_borrow_mut(|state| {
        let status = state
            .project_status
            .get(&hash_id.clone())
            .ok_or("Status cannot be determined".to_string())?;
        if status > &ProjectStatus::FUNDED {
            return Err("Project cannot accept Milestone at this stage".to_string());
        };
        let project = state
            .projects
            .get_mut(&hash_id)
            .ok_or("Project not found")?;

        if project.owner != caller {
            return Err("Only the project owner can add milestones".to_string());
        }

        let total_milestone_amount: u64 = milestones.iter().map(|m| m.payment_amount).sum::<u64>();
        if total_milestone_amount > project.amount_funded {
            return Err("Total milestone amount exceeds project funded amount".to_string());
        }

        project.milestones = milestones;
        Ok(())
    })
}

// API 3: Create a proposal to project by member
#[update(guard = "is_member")]
fn create_project_proposal(hash_id: String, mut proposal: Proposal) -> Result<(), String> {
    let caller = ic_cdk::caller();
    STATE.with_borrow_mut(|state| -> Result<(), String> {
        let project = state
            .projects
            .get_mut(&hash_id)
            .ok_or("Project not found")?;

        let status_opt = state.project_status.get(&hash_id);

        if status_opt.is_none() {
            return  Err("Project Status cannot be determined".to_string());
        }

        let status = status_opt.unwrap();

        if status != &ProjectStatus::PUBLISHED {
            return Err("Project is not accepting Proposals".to_string());
        }

        // Check if the member has the required skillset
        let set = state
            .skillset_by_member
            .get(&caller)
            .ok_or("You don't have any skills".to_string())?;

        let has_required_skills = project
            .skill_sets
            .iter()
            .any(|skill| set.contains(&skill.to_lowercase()));
        if !has_required_skills {
            return Err("Member does not have the required skillset".to_string());
        }

        // Check if the project has reached the maximum number of proposals
        let proposals_to_project = state
            .project_proposals
            .entry(hash_id.clone())
            .or_insert(vec![]);

        if proposals_to_project.len() >= 50 {
            return Err("Project has reached the maximum number of proposals".to_string());
        }

        proposal.sender = caller;
        proposal.timestamp = time();
        proposals_to_project.push(proposal);
        Ok(())
    })
}

#[query]
async fn get_project_proposals(hash_id: String) -> Result<Vec<Proposal>, String> {
    is_project_owner(&hash_id, &caller())?;
    STATE.with_borrow(|appState| {
        let proposals_opt = appState.project_proposals.get(&hash_id);
        if proposals_opt.is_none() {
            return Ok(vec![]);
        } else {
            return Ok(proposals_opt.cloned().unwrap());
        }
    })
}

// API 4: Create a member
#[update(guard = "is_member")]
fn create_member(mut member: Member) -> Result<(), String> {
    let caller = ic_cdk::caller();
    member.principal = caller;
    STATE.with_borrow_mut(|state| {
        state.members.insert(caller, member);
    });
    Ok(())
}

// Get member
#[query]
fn get_member(principal: Principal) -> Option<Member> {
    STATE.with_borrow(|state| state.members.get(&principal).cloned())
}
// API 5: Assign member to Project Model by owner
#[update]
fn assign_member_to_milestone(
    hash_id: String,
    member_principal: Principal,
    milestone_index: i32,
) -> Result<(), String> {
    let caller = ic_cdk::caller();
    STATE.with_borrow_mut(|state| {
        let project = state
            .projects
            .get_mut(&hash_id)
            .ok_or("Project not found")?;

        if project.owner != caller {
            return Err("Only the project owner can assign members".to_string());
        }

        let member = state
            .member_status
            .get_mut(&member_principal)
            .ok_or("Member not found")?;

        if let MemberStatus::UNAVAILABLE = *member {
            return Err("Member is not available".to_string());
        }

        *member = MemberStatus::UNAVAILABLE;

        let member_skillset = state
            .skillset_by_member
            .entry(member_principal)
            .or_insert(HashSet::new());
        // Decrement available skillset count
        for skill in member_skillset.iter() {
            if let Some(count) = state.skillsets_supply.get_mut(&skill.to_lowercase()) {
                *count = count.saturating_sub(1);
            }
        }

        let worket_set = state
            .project_workers
            .entry(hash_id.clone())
            .or_insert(HashSet::new());
        worket_set.insert(member_principal);

        if milestone_index >= 0 {
            let milestone = project
                .milestones
                .get_mut(milestone_index as usize)
                .ok_or("Milestone index not found".to_string())?;
            milestone.member = member_principal;
        }

        Ok(())
    })
}

#[query]
async fn get_project_workers(hash_id: String) -> Vec<Principal> {
    STATE.with_borrow(|appState| {
        let workers_opt = appState.project_workers.get(&hash_id);
        if workers_opt.is_none() {
            return vec![];
        } else {
            return workers_opt.unwrap().into_iter().copied().collect();
        }
    })
}
// API 7: Complete task call to Project
#[update]
async fn complete_milestone(hash_id: String, milestone_index: usize) -> Result<(), String> {
    is_project_owner(&hash_id, &caller())?;
    let (icrc_address, milestone_member, milestone_amt) =
        STATE.with_borrow_mut(|state| -> Result<(String, Principal, u64), String> {
            let project = state
                .projects
                .get_mut(&hash_id)
                .ok_or("Project not found".to_string())?;

            let milestone = project
                .milestones
                .get_mut(milestone_index)
                .ok_or("Milestone not found".to_string())?;

            if let MilestoneStatus::Done = milestone.milestone_status {
                return Err("Milestone has already been completed".to_string());
            }

            Ok((
                project.icrc_token_address.clone(),
                milestone.member,
                milestone.payment_amount,
            ))
        })?;

    let to_acc = Account {
        owner: id(),
        subaccount: Some(principal_to_subaccount(&milestone_member)),
    };
    let sub_acc_array: [u8; 32] = hex::decode(hash_id.clone()).unwrap().try_into().unwrap();

    let _ = disburse_payment(
        Principal::from_text(icrc_address).unwrap(),
        milestone_amt,
        Some(sub_acc_array),
        to_acc,
    )
    .await
    .unwrap();

    STATE.with_borrow_mut(|state| -> Result<(), String> {
        let project = state
            .projects
            .get_mut(&hash_id)
            .ok_or("Project not found".to_string())
            .unwrap();

        let milestone = project
            .milestones
            .get_mut(milestone_index)
            .ok_or("Milestone not found".to_string())
            .unwrap();

        milestone.milestone_status = MilestoneStatus::Done;
        let has_uncomplete_milestone =
            project.milestones.iter().any(|a| match a.milestone_status {
                MilestoneStatus::Undone | MilestoneStatus::InProgress => true,
                _ => {
                    return false;
                }
            });

        if !has_uncomplete_milestone {
            // all mistones is completed
            let project_status = state
                .project_status
                .get_mut(&hash_id)
                .ok_or("Project Status not found".to_string())?;
            *project_status = ProjectStatus::COMPLETED;

            // freeing all workers and adding their skillset back to skill pool
            let worker_set = state
                .project_workers
                .get(&hash_id)
                .ok_or("No workers".to_string())?;

            for worker in worker_set {
                let member_status = state.member_status.get_mut(worker).unwrap();
                *member_status = MemberStatus::AVAILABLE;
                let member_skillset = state
                    .skillset_by_member
                    .entry(worker.clone())
                    .or_insert(HashSet::new());
                // Increment available skillset count
                for skill in member_skillset.iter() {
                    if let Some(count) = state.skillsets_supply.get_mut(&skill.to_lowercase()) {
                        *count = count.saturating_add(1);
                    }
                }
            }
        };

        Ok(())
    })?;

    Ok(())
}

#[update]
async fn manual_payout(
    hash_id: String,
    payout_list: Vec<(Principal, u64)>,
) -> Result<Vec<(Principal, u64)>, String> {
    is_project_owner(&hash_id, &caller())?;
    let mut paid_worker = vec![];
    let subaccount: [u8; 32] = hex::decode(hash_id.clone()).unwrap().try_into().unwrap();
    let sum: u64 = payout_list
        .iter()
        .fold(0, |a, b| a + b.1 + NATIVE_TOKEN_TRANSFER_FEE);
    let check_account = Account {
        owner: id(),
        subaccount: Some(subaccount),
    };
    let balance = check_icrc_balance(
        Principal::from_text(NATIVE_TOKEN_ID).unwrap(),
        check_account,
    )
    .await?;
    if sum > balance {
        return Err("Insufficient fund to payout".to_string());
    };
    for (member, amount) in payout_list {
        STATE.with_borrow(|state| -> Result<(), String> {
            let worker_set = state
                .project_workers
                .get(&hash_id)
                .ok_or("No Worker was assigned".to_string())?;
            if !worker_set.contains(&member) {
                return Err("Worker principal not found.".to_string());
            };
            Ok(())
        })?;

        let to_account = Account {
            owner: id(),
            subaccount: Some(principal_to_subaccount(&member)),
        };
        transfer_to_account(
            Principal::from_text(NATIVE_TOKEN_ID).unwrap(),
            amount,
            Some(subaccount),
            to_account,
        )
        .await?;
        paid_worker.push((member, amount))
    }

    Ok(paid_worker)
}
// API 8: Rent skillset keyword by member duration in months
#[update(guard = "is_member")]
async fn rent_skillset(skillset: String, duration: u8) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let skillset = skillset.to_lowercase();
    STATE.with_borrow_mut(|state| -> Result<(), String> {
        let set = state
            .skillset_by_member
            .entry(caller)
            .or_insert(HashSet::new());
        if set.contains(&skillset) {
            return Err("You have this skill".to_string());
        }

        Ok(())
    })?;
    let current_price = price_of_skillset(skillset.to_lowercase().clone()).await;
    let actual_price = current_price * duration as u64;
    transfer_to_purse(
        Principal::from_text(NATIVE_TOKEN_ID.to_string()).unwrap(),
        actual_price,
        Some(principal_to_subaccount(&caller)),
    )
    .await?;

    STATE.with_borrow_mut(|state| {
        let set = state
            .skillset_by_member
            .entry(caller)
            .or_insert(HashSet::new());
        if !set.contains(&skillset) {
            set.insert(skillset.clone());
        }

        let elapse = days_to_milliseconds(30 * duration as u64);
        let expiration = (ic_cdk::api::time() / 1_000_000) + elapse;
        let expiry_resourse = ExpiryResource {
            expiration,
            data: Some(ExpiryData {
                owner: caller,
                skill: skillset.clone(),
            }),
        };
        SORTED_DATA.with_borrow_mut(|root_note| root_note.insert(expiry_resourse));

        let set = state
            .skillset_by_member
            .entry(caller)
            .or_insert(HashSet::new());

        set.insert(skillset.clone());

        let count = state
            .skillsets_supply
            .entry(skillset.to_lowercase())
            .or_insert(0);
        *count += 1;

        Ok(())
    })
}

#[update]
async fn price_of_skillset(skillset: String) -> u64 {
    let skillset = skillset.to_lowercase();
    let number_project_with_skill = STATE.with_borrow_mut(|state| {
        state
            .skillsets_demand
            .entry(skillset.clone())
            .or_insert(0)
            .clone()
    });

    let available_ppl_with_skill =
        STATE.with_borrow_mut(|state| state.skillsets_supply.entry(skillset).or_insert(0).clone());
    let price_in_dollar = 5f32 + (0.12f32 * number_project_with_skill as f32)
        - (0.6 * available_ppl_with_skill as f32);

    let token_amount = get_amount_of_token_per_dollar(
        NATIVE_TOKEN_SYMBOL.to_string(),
        NATIVE_TOKEN_DECIMAL as usize,
    )
    .await
    .unwrap();
    let tokens = token_amount as f32 * price_in_dollar as f32;
    tokens as u64
}
#[query(guard = "not_anonymous")]
#[candid_method(query)]
fn get_dashboard_metric() -> DashboardMetric {
    let metric = STATE.with_borrow(|state| state.user_dmetrics.get(&caller()).cloned());
    if metric.is_none() {
        return DashboardMetric::default();
    } else {
        return metric.unwrap();
    }
}

#[query]
#[candid_method(query)]
fn get_system_skillsets() -> Vec<String> {
    STATE.with_borrow(|state| state.skillsets_supply.keys().cloned().collect())
}

#[update(guard = "not_anonymous")]
#[candid_method(update)]
async fn transfer_token(token_id: String, amount: u64, address: String) -> Result<Nat, String> {
    let token_principal = Principal::from_text(token_id.clone()).unwrap();

    let user_account = Account {
        owner: id(),
        subaccount: Some(principal_to_subaccount(&caller())),
    };
    // check balance logic
    let (balance,): (Nat,) = call(token_principal, "icrc1_balance_of", (user_account,))
        .await
        .unwrap();

    if amount > balance {
        return Err("Insufficient Funds".to_string());
    }

    let destination_account = Account::from_str(&address).map_err(|err| err.to_string())?;

    let (resp,): (Vec<(String, MetadataValue)>,) =
        call(token_principal, "icrc1_metadata", ()).await.unwrap();
    let mut metadataInfo = ("".to_string(), 0usize, 0usize); // (LOGO, token decimal, fee)

    // this function makes changes to metadatainfo
    process_metadata(resp, &mut metadataInfo);

    let exchange_rate = get_amount_of_token_per_dollar(metadataInfo.0, metadataInfo.1).await?;

    let network_fee = exchange_rate + (2.50f64 * metadataInfo.2 as f64) as u64; // app fee + transfer fee.
    let actual_sent_amt = amount - network_fee as u64;
    let args = TransferArg {
        from_subaccount: Some(principal_to_subaccount(&caller())),
        to: destination_account,
        fee: None,
        created_at_time: None,
        memo: None,
        amount: Nat::from(actual_sent_amt),
    };
    let (resp,): (Result<Nat, TransferError>,) =
        call_with_payment(token_principal, "icrc1_transfer", (args,), 20_000_000)
            .await
            .unwrap();
    let nat_amount = resp.map_err(|err| err.to_string())?;

    transfer_to_purse(
        token_principal,
        network_fee,
        Some(principal_to_subaccount(&caller())),
    )
    .await;
    // Book-keeping
    STATE.with_borrow_mut(|state| {
        if !state.token_used.contains(&token_id) {
            state.token_used.insert(token_id.clone());
        }
    });

    Ok(nat_amount)
}

fn is_project_owner(hash_id: &str, caller: &Principal) -> Result<(), String> {
    STATE.with_borrow(|state| {
        let f = state.project_manager.get(caller);
        if f.is_none() {
            return Err("You are not Authorized".to_string());
        };

        let set = f.unwrap();
        if set.contains(hash_id) {
            return Ok(());
        } else {
            return Err("You are not authorized".to_string());
        }
    })
}

fn not_anonymous() -> Result<(), String> {
    if caller() == Principal::anonymous() {
        return Err("User is not authenticated".to_string());
    }

    Ok(())
}

fn is_member() -> Result<(), String> {
    not_anonymous()?;
    is_user_staked()?;
    Ok(())
}
// Candid export
ic_cdk::export_candid!();
