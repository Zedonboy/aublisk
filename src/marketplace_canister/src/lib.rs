use std::str::FromStr;

use app_core::{config::{AUBLISK_ICRC1_CANISTER_ID, AUBLISK_MARKET_CANISTER_ID, AUBLISK_NFT_CANISTER_ID}, types::{NftError, TokenIdentifier, TokenMetadata}};
use candid::{candid_method, Nat, Principal};
use ic_cdk::{api::{ time}, caller, query, update};
use ic_ledger_types::{AccountIdentifier, Subaccount, TransferError};
use market::{SellOrder};
use types::{MarketError, TransferArgs};
mod types {
    use candid::{CandidType, Nat};
    use ic_ledger_types::{AccountIdentifier, Subaccount};
    use serde::Deserialize;

    #[derive(CandidType)]
    pub enum MarketError {
        UnauthorizedOwner,
        UnauthorizedOperator,
        OwnerNotFound,
        OperatorNotFound,
        TokenNotFound,
        ExistedNFT,
        SelfApprove,
        SelfTransfer,
        TokenInvalid,
        UserNotPremium,
        InsufficientFund,
        InvalidOperation, // Other(String), // for debugging
    }

    #[derive(CandidType, Deserialize)]
    pub struct TransferArgs {
        pub from_subaccount: Subaccount,
        pub to: AccountIdentifier,
        pub amount: Nat,
        pub fee: Option<Nat>,
        pub memo: Option<Vec<u8>>,
        pub created_at_time: Option<Nat>,
    }
}
mod market {
    use app_core::types::{TokenIdentifier, TokenMetadata};
    use candid::{CandidType, Deserialize, Nat, Principal};
    use std::{cell::RefCell, collections::HashMap};

    pub type Price = Nat;

    pub fn with<T, F: FnOnce(&Market) -> T>(f: F) -> T {
        Market.with(|ledger| f(&ledger.borrow()))
    }

    pub fn with_mut<T, F: FnOnce(&mut Market) -> T>(f: F) -> T {
        Market.with(|ledger| f(&mut ledger.borrow_mut()))
    }
    
    // pub fn with_mut<T, F: FnOnce(&mut Market) -> T>(f: F) -> T {
    //     Market.with(|ledger| f(&mut ledger.borrow_mut()))
    // }
    #[derive(CandidType, Deserialize, Clone)]
    pub struct SellOrder {
        pub price: Nat,
        pub prev_owner: Principal,
        pub token_metadata: TokenMetadata
    }

    #[derive(CandidType, Default, Deserialize)]
    pub struct Market {
        pub sell_orders: HashMap<TokenIdentifier, SellOrder>,
    }

    thread_local!(
        static Market: RefCell<Market> = RefCell::new(Market::default());
    );
}

#[query]
#[candid_method(query)]
async fn get_nft_deposit_address() -> String {
    let caller = caller();
    let nft_canister =
        Principal::from_text(AUBLISK_NFT_CANISTER_ID).expect("Invalid NFT canister id");
    let account_id = AccountIdentifier::new(&nft_canister, &Subaccount::from(caller));
    return account_id.to_hex();
}

#[query]
#[candid_method(query)]
async fn check_nft_in_marketplace_address(token_id: TokenIdentifier) -> bool {
    let caller = caller();
    let nft_canister =
        Principal::from_text(AUBLISK_NFT_CANISTER_ID).expect("Invalid NFT canister id");
    let account_id = AccountIdentifier::new(&nft_canister, &Subaccount::from(caller));
    let (nft_possession_rslt,): (bool,) = ic_cdk::call(
        nft_canister,
        "check_account_id_for_ownership",
        (account_id.to_hex(), token_id),
    )
    .await
    .unwrap();
    return nft_possession_rslt;
}

#[query]
#[candid_method(query)]
async fn list_nfts() -> Vec<TokenMetadata> {
    let caller = caller();
    let nft_canister =
        Principal::from_text(AUBLISK_NFT_CANISTER_ID).expect("Invalid NFT canister id");
    let account_id = AccountIdentifier::new(&nft_canister, &Subaccount::from(caller));
    let (nft_possession_rslt,): (Vec<TokenMetadata>,) =
        ic_cdk::call(nft_canister, "get_nfts", (account_id.to_hex(),))
            .await
            .unwrap();
    return nft_possession_rslt;
}

#[query]
async fn get_nfts_on_sale() -> Vec<SellOrder> {
    market::with(|market| {
        market.sell_orders.values().cloned().collect()
    })
}

#[update]
#[candid_method(update)]
async fn sell(token_id: TokenIdentifier, price: Nat) -> Result<(), MarketError> {
    let resp = check_nft_in_marketplace_address(token_id.clone()).await;
    let nft_canister_id = Principal::from_str(AUBLISK_NFT_CANISTER_ID).unwrap();
    let (token_metadata_rslt,):(Result<TokenMetadata, NftError>,) = ic_cdk::call(nft_canister_id, "dip721_token_metadata", (token_id.clone(), )).await.unwrap();
    let token_metadata = token_metadata_rslt.unwrap();
    market::with_mut(|market| {
        if market.sell_orders.contains_key(&token_id) {
            Err(MarketError::InvalidOperation)
        } else {
            if resp {
                market.sell_orders.insert(
                    token_id,
                    SellOrder {price,prev_owner:caller(), token_metadata }
                );
                Ok(())
            } else {
                Err(MarketError::InvalidOperation)
            }
        }
    })
}

#[update]
#[candid_method(update)]
async fn buy(token_id: TokenIdentifier) -> Result<(), MarketError> {
    let person = caller();

    let icrc_canister =
        Principal::from_text(AUBLISK_ICRC1_CANISTER_ID).expect("Canister Id is invalid");
    let market_principal = Principal::from_text(AUBLISK_NFT_CANISTER_ID).unwrap();
    let account = AccountIdentifier::new(&market_principal, &Subaccount::from(person));
    let (user_balance,): (Nat,) = ic_cdk::call(icrc_canister, "icrc1_balance_of", (account,))
        .await
        .unwrap();
    market::with_mut(|market| {
        if market.sell_orders.contains_key(&token_id) {
            let order = market
                .sell_orders
                .get(&token_id)
                .ok_or(MarketError::TokenNotFound)?;

            if user_balance >= order.price {
                let transfer_args = TransferArgs {
                    from_subaccount: Subaccount::from(person),
                    to: AccountIdentifier::new(&market_principal, &Subaccount::from(order.prev_owner)),
                    amount: order.price.clone(),
                    fee: None,
                    memo: None,
                    created_at_time: Some(Nat::from(time()))
                };

                ic_cdk::spawn(transfer(icrc_canister, transfer_args, token_id.clone()));
                market.sell_orders.remove(&token_id);
                // transfer token to owner
                // transfer nft to person
                Ok(())
            } else {
                Err(MarketError::InsufficientFund)
            }
        } else {
            Err(MarketError::InvalidOperation)
        }
    })
}

#[query]
#[candid_method(query)]
fn export_candid() -> String {
    ic_cdk::export_candid!();
    __export_service()
}

async fn transfer(icrc_canister : Principal, t_args : TransferArgs, token_id: TokenIdentifier) {

    let (reslt,) : (Result<Nat, TransferError>, ) = ic_cdk::call(icrc_canister, "icrc1_transfer", (t_args, )).await.expect("");

    reslt.expect("Error");

    let ledger_principal = Principal::from_str(AUBLISK_NFT_CANISTER_ID).unwrap();
    let market_pricipal = Principal::from_str(AUBLISK_MARKET_CANISTER_ID).unwrap();
    let to_account_id = AccountIdentifier::new(&caller(), &Subaccount::from(Principal::anonymous())).to_hex();
    let (reslt,) : (Result<Nat, TransferError>, ) = ic_cdk::call(ledger_principal, "transfer_to_account_id", (market_pricipal, to_account_id,token_id )).await.expect("");

    reslt.expect("Error");
}


