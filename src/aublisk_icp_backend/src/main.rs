use app_core::types::{NftError, TokenIdentifier, TokenMetadata};
use candid::{candid_method, Nat, Principal};
use compile_time_run::run_command_str;
use ic_cdk::api::call::{self, ManualReply};
use ic_cdk::api::{caller, canister_balance128, time, trap};
// use ic_cdk::export::candid::{candid_method, CandidType, Deserialize, Int, Nat};
// use ic_cdk::export::Principal;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use ic_ledger_types::{self, AccountIdentifier, Subaccount};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Not;
use app_core::{self, is_premium_user};
use crate::app_core::types::GenericValue;
use types::*;

mod legacy;

const MAX_COLLECTION_FOR_FREE : usize = 6;
pub mod types {

    use std::default;

    use candid::{CandidType, Int, Nat, Principal};
    use serde::Deserialize;

    use super::*;
    
    #[derive(CandidType, Deserialize, Default, Clone)]
    pub enum Extension {
        #[default]
        DEFAULT,
        PDF,
        TEXT
    }

    #[derive(CandidType, Deserialize, Default, Clone)]
    pub enum NftType {
        #[default]
        ARTICLE,
        SERIES,
        COLLECTION
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub enum NftDataValue {
        NftDataVec(Vec<NFTData>),
        BlobContent(Vec<u8>)
    }

    #[derive(CandidType, Deserialize)]
    pub struct Publisher {
        pub subscibers : HashMap<String, Vec<String>>
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct NFTData {
        pub extension : Extension,
        pub nft_type: NftType,
        pub content: NftDataValue,
        // pub title : String,
        // pub desciption
    }

    
    #[derive(CandidType, Deserialize)]
    pub struct InitArgs {
        pub name: Option<String>,
        pub logo: Option<String>,
        pub symbol: Option<String>,
        pub custodians: Option<HashSet<Principal>>,
        pub cap: Option<Principal>,
    }
    #[derive(CandidType, Default, Deserialize)]
    pub struct Metadata {
        pub name: Option<String>,
        pub logo: Option<String>,
        pub symbol: Option<String>,
        pub custodians: HashSet<Principal>,
        pub created_at: u64,
        pub upgraded_at: u64,
    }
    #[derive(CandidType)]
    pub struct Stats {
        pub total_transactions: Nat,
        pub total_supply: Nat,
        pub cycles: Nat,
        pub total_unique_holders: Nat,
    }
   
    #[derive(CandidType)]
    pub enum SupportedInterface {
        Approval,
        Mint,
        Burn,
    }
    
}

mod ledger {
    use app_core::types::{NftError, TokenIdentifier, TokenMetadata};
    use candid::CandidType;
    use ic_ledger_types::AccountIdentifier;
    use serde::Deserialize;

    use super::*;
    thread_local!(
        static LEDGER: RefCell<Ledger> = RefCell::new(Ledger::default());
    );

    pub fn with<T, F: FnOnce(&Ledger) -> T>(f: F) -> T {
        LEDGER.with(|ledger| f(&ledger.borrow()))
    }

    pub fn with_mut<T, F: FnOnce(&mut Ledger) -> T>(f: F) -> T {
        LEDGER.with(|ledger| f(&mut ledger.borrow_mut()))
    }

    #[derive(CandidType, Default, Deserialize)]
    pub struct Ledger {
        pub series_subscribers: HashMap<TokenIdentifier, Publisher>,
        pub metadata: Metadata,
        pub nft_datas: HashMap<TokenIdentifier, NFTData>,
        pub tokens: HashMap<TokenIdentifier, TokenMetadata>, // recommend to have sequential id
        pub owners: HashMap<Principal, HashSet<TokenIdentifier>>, // quick lookup
        pub owners_as_account_id: HashMap<AccountIdentifier, HashSet<TokenIdentifier>>,
        pub operators: HashMap<Principal, HashSet<TokenIdentifier>>, // quick lookup
        pub tx_count: Nat,
    }

    impl Ledger {
        pub fn init_metadata(&mut self, default_custodian: Principal, args: Option<InitArgs>) {
            let metadata = self.metadata_mut();
            metadata.custodians.insert(default_custodian);
            if let Some(args) = args {
                metadata.name = args.name;
                metadata.logo = args.logo;
                metadata.symbol = args.symbol;
                if let Some(custodians) = args.custodians {
                    for custodians in custodians {
                        metadata.custodians.insert(custodians);
                    }
                }

                // initiate cap with specified canister, otherwise use mainnet canister
                // handshake(1_000_000_000_000, args.cap);
            } else {
                // default to mainnet cap canister if no args are specified
                // handshake(1_000_000_000_000, None);
            }
            metadata.created_at = time();
            metadata.upgraded_at = time();
        }

        pub fn metadata(&self) -> &Metadata {
            &self.metadata
        }

        pub fn metadata_mut(&mut self) -> &mut Metadata {
            &mut self.metadata
        }

        pub fn tokens_count(&self) -> usize {
            self.tokens.len()
        }

        pub fn tx_count(&self) -> Nat {
            self.tx_count.clone()
        }

        pub fn is_token_existed(&self, token_identifier: &TokenIdentifier) -> bool {
            self.tokens.contains_key(token_identifier)
        }

        pub fn token_metadata(
            &self,
            token_identifier: &TokenIdentifier,
        ) -> Result<&TokenMetadata, NftError> {
            self.tokens
                .get(token_identifier)
                .ok_or(NftError::TokenNotFound)
        }

        pub fn add_token_metadata(
            &mut self,
            token_identifier: TokenIdentifier,
            token_metadata: TokenMetadata,
        ) {
            self.tokens.insert(token_identifier, token_metadata);
        }

        pub fn account_id_nft_list(&self, a : AccountIdentifier) -> Vec<TokenIdentifier> {
            let token_list: Vec<TokenIdentifier> = self.owners_as_account_id.get(&a).expect("Not Found").iter().cloned().collect();
            token_list
        }

        pub fn add_nft_data(&mut self,  token_identifier: TokenIdentifier,
            nft_data: NFTData,) {
                self.nft_datas.insert(token_identifier, nft_data);
            }

        pub fn nft_data(&self, token_identifier : &TokenIdentifier) -> Result<&NFTData, NftError> {
            self.nft_datas.get(token_identifier).ok_or(NftError::TokenNotFound)
        }

        pub fn add_to_series_nft_data(&mut self, token_identifier : &TokenIdentifier, nft_data : NFTData) -> Result<(), NftError> {
            let series = self.nft_datas.get_mut(token_identifier).ok_or(NftError::TokenNotFound)?;
            if let NftType::SERIES = series.nft_type {
                match &mut series.content {
                    NftDataValue::NftDataVec(data_vec) => {
                       
                        data_vec.push(nft_data);
                        return Ok(())
                    }
                    _ => return Err(NftError::TokenInvalid),
                } 
            } else {
                return Err(NftError::TokenInvalid)
            }
        }

       

        pub fn notify_series_subscribers(&mut self, token_identifier : &TokenIdentifier, message : String) -> Result<(), NftError> {

            let publisher =  self.series_subscribers.get_mut(token_identifier).ok_or(NftError::InvalidOperation)?;
            for mssg_bag in publisher.subscibers.values_mut() {
                mssg_bag.push(message.clone())
            };

            Ok(())

        }

        pub fn owners_count(&self) -> usize {
            self.owners.len()
        }

        pub fn owner_token_identifiers(
            &self,
            owner: &Principal,
        ) -> Result<&HashSet<TokenIdentifier>, NftError> {
            self.owners.get(owner).ok_or(NftError::OwnerNotFound)
        }

        pub fn owner_of(
            &self,
            token_identifier: &TokenIdentifier,
        ) -> Result<Option<Principal>, NftError> {
            self.token_metadata(token_identifier)
                .map(|token_metadata| token_metadata.owner)
        }
        pub fn account_id_is_nft_owner(&self, account : &AccountIdentifier, token_identifier : &TokenIdentifier) -> bool{
            let token_list = self.owners_as_account_id.get(account).expect("Owner not found");
            let v = token_list.contains(token_identifier);
            return v;
        }

        pub fn owner_token_metadata(
            &self,
            owner: &Principal,
        ) -> Result<Vec<&TokenMetadata>, NftError> {
            self.owner_token_identifiers(owner)?
                .iter()
                .map(|token_identifier| self.token_metadata(token_identifier))
                .collect()
        }

        pub fn update_owner_cache(
            &mut self,
            token_identifier: &TokenIdentifier,
            old_owner: Option<Principal>,
            new_owner: Option<Principal>,
        ) {
            if let Some(old_owner) = old_owner {
                let old_owner_token_identifiers = self
                    .owners
                    .get_mut(&old_owner)
                    .expect("couldn't find owner");
                old_owner_token_identifiers.remove(token_identifier);
                if old_owner_token_identifiers.is_empty() {
                    self.owners.remove(&old_owner);
                }
            }
            if let Some(new_owner) = new_owner {
                self.owners
                    .entry(new_owner)
                    .or_insert_with(HashSet::new)
                    .insert(token_identifier.clone());
            }
        }

        pub fn operator_token_identifiers(
            &self,
            operator: &Principal,
        ) -> Result<&HashSet<TokenIdentifier>, NftError> {
            self.operators
                .get(operator)
                .ok_or(NftError::OperatorNotFound)
        }

        pub fn operator_of(
            &self,
            token_identifier: &TokenIdentifier,
        ) -> Result<Option<Principal>, NftError> {
            self.token_metadata(token_identifier)
                .map(|token_metadata| token_metadata.operator)
        }

        pub fn operator_token_metadata(
            &self,
            operator: &Principal,
        ) -> Result<Vec<&TokenMetadata>, NftError> {
            self.operator_token_identifiers(operator)?
                .iter()
                .map(|token_identifier| self.token_metadata(token_identifier))
                .collect()
        }

        pub fn update_operator_cache(
            &mut self,
            token_identifier: &TokenIdentifier,
            old_operator: Option<Principal>,
            new_operator: Option<Principal>,
        ) {
            if let Some(old_operator) = old_operator {
                let old_operator_token_identifiers = self
                    .operators
                    .get_mut(&old_operator)
                    .expect("couldn't find operator");
                old_operator_token_identifiers.remove(token_identifier);
                if old_operator_token_identifiers.is_empty() {
                    self.operators.remove(&old_operator);
                }
            }
            if let Some(new_operator) = new_operator {
                self.operators
                    .entry(new_operator)
                    .or_insert_with(HashSet::new)
                    .insert(token_identifier.clone());
            }
        }

        pub fn approve(
            &mut self,
            approved_by: Principal,
            token_identifier: &TokenIdentifier,
            new_operator: Option<Principal>,
        ) {
            let token_metadata = self
                .tokens
                .get_mut(token_identifier)
                .expect("couldn't find token metadata");
            token_metadata.operator = new_operator;
            token_metadata.approved_by = Some(approved_by);
            token_metadata.approved_at = Some(time());
        }

        pub fn transfer(
            &mut self,
            transferred_by: Principal,
            token_identifier: &TokenIdentifier,
            new_owner: Option<Principal>,
        ) {
            let token_metadata = self
                .tokens
                .get_mut(token_identifier)
                .expect("couldn't find token metadata");
            token_metadata.owner = new_owner;
            token_metadata.transferred_by = Some(transferred_by);
            token_metadata.transferred_at = Some(time());
            token_metadata.operator = None;
        }

        pub fn burn(&mut self, burned_by: Principal, token_identifier: &TokenIdentifier) {
            let token_metadata = self
                .tokens
                .get_mut(token_identifier)
                .expect("couldn't find token metadata");
            token_metadata.owner = None;
            token_metadata.operator = None;
            token_metadata.is_burned = true;
            token_metadata.burned_by = Some(burned_by);
            token_metadata.burned_at = Some(time());
        }

        pub fn inc_tx(&mut self) -> Nat {
            self.tx_count += 1u32;
            self.tx_count.clone()
        }
    }
}

#[init]
#[candid_method(init)]
fn init(args: Option<InitArgs>) {
    ledger::with_mut(|ledger| ledger.init_metadata(caller(), args));
}

pub fn is_canister_custodian() -> Result<(), String> {
    ledger::with(|ledger| {
        ledger
            .metadata()
            .custodians
            .contains(&caller())
            .then_some(())
            .ok_or_else(|| "Caller is not an custodian of canister".into())
    })
}

// ==================================================================================================
// cover metadata
// ==================================================================================================
#[query()]
#[candid_method(query)]
fn git_commit_hash() -> &'static str {
    run_command_str!("git", "rev-parse", "HEAD")
}

#[query()]
#[candid_method(query)]
fn rust_toolchain_info() -> &'static str {
    run_command_str!("rustup", "show")
}

#[query()]
#[candid_method(query)]
fn dfx_info() -> &'static str {
    run_command_str!("dfx", "--version")
}

// ==================================================================================================
// metadata
// ==================================================================================================
#[query(manual_reply = true)]
#[candid_method(query)]
fn dip721_name() -> ManualReply<Option<String>> {
    ledger::with(|ledger| ManualReply::one(ledger.metadata().name.as_ref()))
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn dip721_logo() -> ManualReply<Option<String>> {
    ledger::with(|ledger| ManualReply::one(ledger.metadata().logo.as_ref()))
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn dip721_symbol() -> ManualReply<Option<String>> {
    ledger::with(|ledger| ManualReply::one(ledger.metadata().symbol.as_ref()))
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn dip721_custodians() -> ManualReply<HashSet<Principal>> {
    ledger::with(|ledger| ManualReply::one(&ledger.metadata().custodians))
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn dip721_metadata() -> ManualReply<Metadata> {
    ledger::with(|ledger| ManualReply::one(ledger.metadata()))
}

#[update(guard = "is_canister_custodian")]
#[candid_method(update)]
fn dip721_set_name(name: String) {
    ledger::with_mut(|ledger| ledger.metadata_mut().name = Some(name));
}

#[update(guard = "is_canister_custodian")]
#[candid_method(update)]
fn dip721_set_logo(logo: String) {
    ledger::with_mut(|ledger| ledger.metadata_mut().logo = Some(logo));
}

#[update(guard = "is_canister_custodian")]
#[candid_method(update)]
fn dip721_set_symbol(symbol: String) {
    ledger::with_mut(|ledger| ledger.metadata_mut().symbol = Some(symbol));
}

#[update(guard = "is_canister_custodian")]
#[candid_method(update)]
fn dip721_set_custodians(custodians: HashSet<Principal>) {
    ledger::with_mut(|ledger| ledger.metadata_mut().custodians = custodians);
}

// ==================================================================================================
// stats
// ==================================================================================================
/// Returns the total current supply of NFT tokens.
/// NFTs that are minted and later burned explicitly or sent to the zero address should also count towards totalSupply.
#[query()]
#[candid_method(query)]
fn dip721_total_supply() -> Nat {
    ledger::with(|ledger| Nat::from(ledger.tokens_count()))
}

#[query()]
#[candid_method(query)]
fn dip721_total_transactions() -> Nat {
    ledger::with(|ledger| ledger.tx_count())
}

#[query()]
#[candid_method(query)]
fn dip721_cycles() -> Nat {
    Nat::from(canister_balance128())
}

#[query()]
#[candid_method(query)]
fn dip721_total_unique_holders() -> Nat {
    ledger::with(|ledger| Nat::from(ledger.owners_count()))
}

#[query()]
#[candid_method(query)]
fn dip721_stats() -> Stats {
    Stats {
        total_transactions: dip721_total_transactions(),
        total_supply: dip721_total_supply(),
        cycles: dip721_cycles(),
        total_unique_holders: dip721_total_unique_holders(),
    }
}

// ==================================================================================================
// supported interfaces
// ==================================================================================================
#[query()]
#[candid_method(query)]
fn dip721_supported_interfaces() -> Vec<SupportedInterface> {
    vec![
        SupportedInterface::Approval,
        SupportedInterface::Mint,
        SupportedInterface::Burn,
    ]
}

// ==================================================================================================
// balance
// ==================================================================================================
#[query()]
#[candid_method(query)]
fn dip721_balance_of(owner: Principal) -> Result<Nat, NftError> {
    ledger::with(|ledger| {
        ledger
            .owner_token_identifiers(&owner)
            .map(|token_identifiers| Nat::from(token_identifiers.len()))
    })
}

// ==================================================================================================
// token ownership
// ==================================================================================================
#[query()]
#[candid_method(query)]
fn dip721_owner_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
    ledger::with(|ledger| ledger.owner_of(&token_identifier))
}

#[query]
fn check_account_id_for_ownership(account_id_hex : String, token_identifier : TokenIdentifier) -> bool {
    let account_identifier = AccountIdentifier::from_hex(&account_id_hex).expect("invalid accoint");
    ledger::with(|ledger| {
        ledger.account_id_is_nft_owner(&account_identifier, &token_identifier)
    })
}

#[query]
fn get_nfts(account_id_hex : String) -> Vec<TokenMetadata> {
    let account_identifier = AccountIdentifier::from_hex(&account_id_hex).expect("invalid accoint");
    ledger::with(|ledger| {
        let mut vec_metadata = vec![];
        for token_id in ledger.account_id_nft_list(account_identifier) {
            let token_metadata = ledger.tokens.get(&token_id);
            if token_metadata.is_none() {
                continue;
            }

            vec_metadata.push(token_metadata.cloned().unwrap())
        } 

        vec_metadata

    })
}

#[query()]
#[candid_method(query)]
fn dip721_operator_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
    ledger::with(|ledger| ledger.operator_of(&token_identifier))
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn dip721_owner_token_metadata(
    owner: Principal,
) -> ManualReply<Result<Vec<TokenMetadata>, NftError>> {
    ledger::with(|ledger| ManualReply::one(ledger.owner_token_metadata(&owner)))
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn dip721_operator_token_metadata(
    operator: Principal,
) -> ManualReply<Result<Vec<TokenMetadata>, NftError>> {
    ledger::with(|ledger| ManualReply::one(ledger.operator_token_metadata(&operator)))
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn dip721_owner_token_identifiers(
    owner: Principal,
) -> ManualReply<Result<Vec<TokenIdentifier>, NftError>> {
    ledger::with(|ledger| ManualReply::one(ledger.owner_token_identifiers(&owner)))
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn dip721_operator_token_identifiers(
    operator: Principal,
) -> ManualReply<Result<Vec<TokenIdentifier>, NftError>> {
    ledger::with(|ledger| ManualReply::one(ledger.operator_token_identifiers(&operator)))
}

// ==================================================================================================
// token metadata
// ==================================================================================================
#[query(manual_reply = true)]
#[candid_method(query)]
fn dip721_token_metadata(
    token_identifier: TokenIdentifier,
) -> ManualReply<Result<TokenMetadata, NftError>> {
    ledger::with(|ledger| ManualReply::one(ledger.token_metadata(&token_identifier)))
}

// ==================================================================================================
// approved for all
// ==================================================================================================
#[query()]
#[candid_method(query)]
fn dip721_is_approved_for_all(owner: Principal, operator: Principal) -> Result<bool, NftError> {
    ledger::with(|ledger| {
        ledger
            .owner_token_metadata(&owner)
            .map(|owner_token_metadata| {
                owner_token_metadata
                    .iter()
                    .all(|token_metadata| token_metadata.operator.eq(&Some(operator)))
            })
    })
}

// ==================================================================================================
// core api
// ==================================================================================================
#[update]
#[candid_method(update)]
fn dip721_approve(operator: Principal, token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = caller();
        operator
            .ne(&caller)
            .then_some(())
            .ok_or(NftError::SelfApprove)?;
        ledger
            .owner_of(&token_identifier)?
            .eq(&Some(caller))
            .then_some(())
            .ok_or(NftError::UnauthorizedOwner)?;
        ledger.update_operator_cache(
            &token_identifier,
            ledger.operator_of(&token_identifier)?,
            Some(operator),
        );
        ledger.approve(caller, &token_identifier, Some(operator));

        // insert_sync(IndefiniteEvent {
        //     caller,
        //     operation: "approve".into(),
        //     details: vec![
        //         ("operator".into(), DetailValue::from(operator)),
        //         (
        //             "token_identifier".into(),
        //             DetailValue::from(token_identifier.to_string()),
        //         ),
        //     ],
        // });

        Ok(ledger.inc_tx() - 1u32)
    })
}

/// since we've supported single operator per owner only
/// so when `is_approved` is false that mean set all caller's nfts to None regardless of `operator`
/// otherwise set all caller's nfts to `operator`
#[update]
#[candid_method(update)]
fn dip721_set_approval_for_all(operator: Principal, is_approved: bool) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = caller();
        operator
            .ne(&caller)
            .then_some(())
            .ok_or(NftError::SelfApprove)?;
        let owner_token_identifiers = ledger.owner_token_identifiers(&caller)?.clone();
        for token_identifier in owner_token_identifiers {
            let old_operator = ledger.operator_of(&token_identifier)?;
            let new_operator = if is_approved { Some(operator) } else { None };
            ledger.update_operator_cache(&token_identifier, old_operator, new_operator);
            ledger.approve(caller, &token_identifier, new_operator);
        }

        // insert_sync(IndefiniteEvent {
        //     caller,
        //     operation: "setApprovalForAll".into(),
        //     details: vec![
        //         ("operator".into(), DetailValue::from(operator)),
        //         (
        //             "is_approved".into(),
        //             if is_approved {
        //                 DetailValue::True
        //             } else {
        //                 DetailValue::False
        //             },
        //         ),
        //     ],
        // });

        Ok(ledger.inc_tx() - 1u32)
    })
}

#[update]
#[candid_method(update)]
fn dip721_transfer(to: Principal, token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = caller();
        to.ne(&caller).then_some(()).ok_or(NftError::SelfTransfer)?;
        let old_owner = ledger.owner_of(&token_identifier)?;
        let old_operator = ledger.operator_of(&token_identifier)?;
        old_owner
            .eq(&Some(caller))
            .then_some(())
            .ok_or(NftError::UnauthorizedOwner)?;
        ledger.update_owner_cache(&token_identifier, old_owner, Some(to));
        ledger.update_operator_cache(&token_identifier, old_operator, None);
        ledger.transfer(caller, &token_identifier, Some(to));

        // insert_sync(IndefiniteEvent {
        //     caller,
        //     operation: "transfer".into(),
        //     details: vec![
        //         ("owner".into(), DetailValue::from(caller)),
        //         ("to".into(), DetailValue::from(to)),
        //         (
        //             "token_identifier".into(),
        //             DetailValue::from(token_identifier.to_string()),
        //         ),
        //     ],
        // });

        Ok(ledger.inc_tx() - 1u32)
    })
}

#[update]
#[candid_method(update)]
async fn transfer_to_account_id(to_principal: Principal, to_account_id: String, token_identifier : TokenIdentifier) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let account_id = AccountIdentifier::from_hex(&to_account_id).unwrap();
        let inc = dip721_transfer(to_principal, token_identifier.clone())?;
        if ledger.owners_as_account_id.contains_key(&account_id) {
            let set = ledger.owners_as_account_id.get_mut(&account_id).unwrap();
            set.insert(token_identifier);
        } else {
            let mut set:HashSet<TokenIdentifier> = HashSet::new();
            set.insert(token_identifier);
            ledger.owners_as_account_id.insert(account_id, set );
        }
        
        Ok(inc)
    })
}

#[update]
#[candid_method(update)]
fn dip721_transfer_from(
    owner: Principal,
    to: Principal,
    token_identifier: TokenIdentifier,
) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = caller();
        owner.ne(&to).then_some(()).ok_or(NftError::SelfTransfer)?;
        let old_owner = ledger.owner_of(&token_identifier)?;
        let old_operator = ledger.operator_of(&token_identifier)?;
        old_owner
            .eq(&Some(owner))
            .then_some(())
            .ok_or(NftError::UnauthorizedOwner)?;
        old_operator
            .eq(&Some(caller))
            .then_some(())
            .ok_or(NftError::UnauthorizedOperator)?;
        ledger.update_owner_cache(&token_identifier, old_owner, Some(to));
        ledger.update_operator_cache(&token_identifier, old_operator, None);
        ledger.transfer(caller, &token_identifier, Some(to));

        // insert_sync(IndefiniteEvent {
        //     caller,
        //     operation: "transferFrom".into(),
        //     details: vec![
        //         ("owner".into(), DetailValue::from(owner)),
        //         ("to".into(), DetailValue::from(to)),
        //         (
        //             "token_identifier".into(),
        //             DetailValue::from(token_identifier.to_string()),
        //         ),
        //     ],
        // });

        Ok(ledger.inc_tx() - 1u32)
    })
}

#[update]
#[candid_method(update)]
async fn dip721_mint(
    to: Principal,
    token_identifier: TokenIdentifier,
    properties: Vec<(String, GenericValue)>,
    nft_data : Option<NFTData>
) -> Result<Nat, NftError> {
    let caller = caller();
    if nft_data.is_none() {
        assert!(false, "This is not a normal nft canister")
    }
    let nft_data = nft_data.expect("Nft data is null");
    match &nft_data.content {
        NftDataValue::BlobContent(data) => {
            if data.len() > 1_000_000 {
                // greater than 1 mb is for Premium users
                let user_amt = is_premium_user(caller).await.expect("Data greater than 1mb is for premium users, and the system cannot determine that.");
                if user_amt == 0u32 {
                    trap("You are not premium user because you have zero balance")
                }
                
            }
        },

        NftDataValue::NftDataVec(data) => {
            if data.len() > MAX_COLLECTION_FOR_FREE {
                let user_amt = is_premium_user(caller).await.expect("Data greater than 1mb is for premium users, and the system cannot determine that.");
                if user_amt == 0u32 {
                    trap("You are not premium user because you have zero balance")
                }
            } else {
                for _nftdata in data {
                    match &_nftdata.content {
                        NftDataValue::BlobContent(data) => {
                            if data.len() > 1_000_000 {
                                // greater than 1 mb is for Premium users
                                let user_amt = is_premium_user(caller).await.expect("Data greater than 1mb is for premium users, and the system cannot determine that.");
                                if user_amt == 0u32 {
                                    trap("You are not premium user because you have zero balance")
                                }
                                
                            }
                        },

                        _  => {
                            trap("You cannot nest nftdata too deep.")
                        }
                    }
                }
            }
        }
    }
    ledger::with_mut(|ledger| {
        
        ledger
            .is_token_existed(&token_identifier)
            .not()
            .then_some(())
            .ok_or(NftError::ExistedNFT)?;

        
        ledger.add_token_metadata(
            token_identifier.clone(),
            TokenMetadata {
                token_identifier: token_identifier.clone(),
                owner: Some(to),
                operator: None,
                properties,
                is_burned: false,
                minted_at: time(),
                minted_by: caller,
                transferred_at: None,
                transferred_by: None,
                approved_at: None,
                approved_by: None,
                burned_at: None,
                burned_by: None,
            },
        );



        ledger.add_nft_data(token_identifier.clone(), nft_data);
        ledger.update_owner_cache(&token_identifier, None, Some(to));

        // insert_sync(IndefiniteEvent {
        //     caller,
        //     operation: "mint".into(),
        //     details: vec![
        //         ("to".into(), DetailValue::from(to)),
        //         (
        //             "token_identifier".into(),
        //             DetailValue::from(token_identifier.to_string()),
        //         ),
        //     ],
        // });

        Ok(ledger.inc_tx() - 1u32)
    })
}

#[update]
#[candid_method(update)]
fn dip721_burn(token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = caller();
        let old_owner = ledger.owner_of(&token_identifier)?;
        old_owner
            .eq(&Some(caller))
            .then_some(())
            .ok_or(NftError::UnauthorizedOwner)?;
        let old_operator = ledger.operator_of(&token_identifier)?;
        ledger.update_owner_cache(&token_identifier, old_owner, None);
        ledger.update_operator_cache(&token_identifier, old_operator, None);
        ledger.burn(caller, &token_identifier);

        // insert_sync(IndefiniteEvent {
        //     caller,
        //     operation: "burn".into(),
        //     details: vec![(
        //         "token_identifier".into(),
        //         DetailValue::from(token_identifier.to_string()),
        //     )],
        // });

        Ok(ledger.inc_tx() - 1u32)
    })
}

#[query]
async fn owner_nftdata() -> Vec<NFTData> {
    ledger::with(|ledger| {
        let set = ledger.owner_token_identifiers(&caller()).unwrap();
        let mut nft_vec = vec![];
        for token_id in set {
            let nftd = ledger.nft_datas.get(token_id).cloned().unwrap();
            nft_vec.push(nftd)
        }
        nft_vec
    })
}

#[update]
#[candid_method(update)]
async fn add_to_series(token_identifier: TokenIdentifier, data : NFTData) -> Result<Nat, NftError> {
    let result_premium = is_premium_user(caller()).await;
    ledger::with_mut(|ledger|{
        let caller = caller();
        let owner = ledger.owner_of(&token_identifier)?;
        owner.eq(&Some(caller)).then_some(()).ok_or(NftError::UnauthorizedOwner)?;
        let nft_data = ledger.nft_data(&token_identifier)?;

        if let NftDataValue::NftDataVec(data_vec) = &nft_data.content {
            if data_vec.len() >= 6 && result_premium.is_err() {
                return Err(NftError::UserNotPremium);
            }
        }

        ledger.add_to_series_nft_data(&token_identifier, data)?;
        let metadata = ledger.token_metadata(&token_identifier)?;
        let binding = token_identifier.clone();
        let element = metadata.properties.iter().find(|&x| x.0 == "title").ok_or(&binding);
        let series_name : String;
        match element {
            Ok((_title, value)) => {
                match value {
                    GenericValue::TextContent(data) => {
                        series_name = data.clone();
                    },

                    _ => {
                        series_name = token_identifier.clone().to_string()
                    }
                }
            },

            Err(tm) => {
                series_name = tm.to_string()
            }
        }
        ledger.notify_series_subscribers(&token_identifier, format!("New Update on {} series", series_name))?;

        Ok(ledger.inc_tx() - 1u32)
    })
}

async fn export_nft() {
    // make HHTP-calls
}

// ==================================================================================================
// upgrade
// ==================================================================================================
/// NOTE:
/// If you plan to store gigabytes of state and upgrade the code,
/// Using stable memory as the main storage is a good option to consider
#[pre_upgrade]
fn pre_upgrade() {
    ledger::with(|ledger| {
        // ic_cdk::storage::stable_save(t)
        if let Err(err) = ic_cdk::storage::stable_save::<(&ledger::Ledger,)>((ledger,)) {
            trap(&format!(
                "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                err
            ));
        };
    })
}

#[post_upgrade]
fn post_upgrade() {
    ledger::with_mut(|ledger| {
        match ic_cdk::storage::stable_restore::<(ledger::Ledger,)>() {
            Ok((ledger_store,)) => {
                *ledger = ledger_store;
                ledger.metadata_mut().upgraded_at = time();
            }
            Err(err) => {
                trap(&format!(
                    "An error occurred when loading from stable memory (post_upgrade): {:?}",
                    err
                ));
            }
        }
    })
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    std::print!("{}", export_candid());
}

#[query]
#[candid_method(query)]
fn export_candid() -> String {
    ic_cdk::export_candid!();
    __export_service()
}

// ROADMAP:
// - notification
// - consider support: multiple operators per owner
