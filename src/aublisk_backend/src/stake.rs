use std::collections::HashMap;

use candid::{CandidType, Principal};
use ic_cdk::{caller, query, update};
use serde::{Deserialize, Serialize};

use crate::utils::transfer_to_staking_purse;
pub const NATIVE_TOKEN_SYMBOL : &str = "";
pub const NATIVE_TOKEN_DECIMAL : u8 = 8;
pub const NATIVE_TOKEN_ID : &str = "";
pub const NATIVE_TOKEN_TRANSFER_FEE : u64 = 10000;
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct Neuron {
    owner: Principal,
    stake: u64,
    dissolve_delay: u64,
    created_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
struct Proposal {
    id: u64,
    proposer: Principal,
    description: String,
    yes_votes: u64,
    no_votes: u64,
    status: ProposalStatus,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
enum ProposalStatus {
    Active,
    Passed,
    Rejected,
}

#[derive(Deserialize, Serialize)]
struct GovernanceState {
    neurons: HashMap<Principal, Neuron>,
    proposals: HashMap<u64, Proposal>,
    next_proposal_id: u64,
}

thread_local! {
    static STATE: std::cell::RefCell<GovernanceState> = std::cell::RefCell::new(GovernanceState {
        neurons: HashMap::new(),
        proposals: HashMap::new(),
        next_proposal_id: 0,
    });
}

#[update]
async fn stake(amount: u64, dissolve_delay: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();

    transfer_to_staking_purse(amount, caller).await?;
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let neuron = Neuron {
            owner: caller,
            stake: amount,
            dissolve_delay,
            created_at: ic_cdk::api::time(),
        };
        state.neurons.insert(caller, neuron);
    });
    Ok(())
}

// #[update]
// fn create_proposal(description: String) -> u64 {
//     let caller = ic_cdk::caller();
//     STATE.with(|state| {
//         let mut state = state.borrow_mut();
//         let proposal_id = state.next_proposal_id;
//         state.next_proposal_id += 1;
//         let proposal = Proposal {
//             id: proposal_id,
//             proposer: caller,
//             description,
//             yes_votes: 0,
//             no_votes: 0,
//             status: ProposalStatus::Active,
//         };
//         state.proposals.insert(proposal_id, proposal);
//         proposal_id
//     })
// }

// #[update]
// fn vote(proposal_id: u64, approve: bool) -> bool {
//     let caller = ic_cdk::caller();
//     STATE.with(|state| {
//         let mut state = state.borrow_mut();
//         if let Some(neuron) = state.neurons.get(&caller) {
//             if let Some(proposal) = state.proposals.get_mut(&proposal_id) {
//                 if approve {
//                     proposal.yes_votes += neuron.stake;
//                 } else {
//                     proposal.no_votes += neuron.stake;
//                 }
//                 true
//             } else {
//                 false
//             }
//         } else {
//             false
//         }
//     })
// }

// #[query]
// fn get_proposal(id: u64) -> Option<Proposal> {
//     STATE.with(|state| state.borrow().proposals.get(&id).cloned())
// }

#[query]
fn get_neuron() -> Option<Neuron> {
    STATE.with(|state| state.borrow().neurons.get(&caller()).cloned())
}

#[query]
fn get_staked_amount() -> u64 {
    let neuron = get_neuron();
    if neuron.is_none() {
        return 0;
    } else {
        return neuron.unwrap().stake;
    }
}

// Helper function to calculate rewards (simplified)
fn calculate_rewards(neuron: &Neuron) -> u64 {
    let now = ic_cdk::api::time();
    let time_staked = now - neuron.created_at;
    (neuron.stake * time_staked as u64) / (365 * 24 * 60 * 60 * 1_000_000_000)
}

#[update]
fn claim_rewards() -> u64 {
    let caller = ic_cdk::caller();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(neuron) = state.neurons.get_mut(&caller) {
            let rewards = calculate_rewards(neuron);
            neuron.stake += rewards;
            rewards
        } else {
            0
        }
    })
}

#[update]
fn unstake() -> u64 {
    //TODO implement unstake logic and funds transfer
    return 0;
    let caller = ic_cdk::caller();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(neuron) = state.neurons.remove(&caller) {
            let now = ic_cdk::api::time();
            if now >= neuron.created_at + neuron.dissolve_delay {
                neuron.stake + calculate_rewards(&neuron)
            } else {
                0
            }
        } else {
            0
        }
    })
}

pub fn is_user_staked() -> Result<(), String> {
    
    STATE.with_borrow(|state| {
        let _ = state.neurons.get(&caller()).ok_or("You have not staked tokens".to_string())?;
        Ok(())
    })
}