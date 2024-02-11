use config::AUBLISK_ICRC1_CANISTER_ID;
use ic_cdk;
use candid::{self, Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;

pub mod config;
pub mod types;
//Premium users are users that have our ICRC-1 token in their account or hold.
pub async fn is_premium_user(user : Principal) -> Result<Nat, String> {
    let icrc_canister = Principal::from_text(AUBLISK_ICRC1_CANISTER_ID).expect("Canister Id is invalid");

    let account = Account{
        owner: user,
        subaccount: None
    };

    let a: Result<(Nat,), _> = ic_cdk::call(icrc_canister, "icrc1_balance_of", (account,)).await;

    if let Ok((amt,)) = a {
        if amt > 0u32 {
            return Ok(amt);
        } else {

            return Err("You have 0 balance".to_string());
        }
        
    } else {
        return Err("Cannot determine if user is premium".to_string());
    }
}