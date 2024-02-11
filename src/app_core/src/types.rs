use candid::{CandidType, Deserialize, Int, Nat, Principal};

pub type TokenIdentifier = String;
    #[derive(CandidType, Deserialize, Clone)]
    pub enum GenericValue {
        BoolContent(bool),
        TextContent(String),
        BlobContent(Vec<u8>),
        Principal(Principal),
        Nat8Content(u8),
        Nat16Content(u16),
        Nat32Content(u32),
        Nat64Content(u64),
        NatContent(Nat),
        Int8Content(i8),
        Int16Content(i16),
        Int32Content(i32),
        Int64Content(i64),
        IntContent(Int),
        FloatContent(f64), // motoko only support f64
        NestedContent(Vec<(String, GenericValue)>),
        
    }

    impl Default for GenericValue {
        fn default() -> Self {
            GenericValue::BoolContent(false)
        }
    }
    /// Please notice that the example of internal data structure as below doesn't represent your final storage, please use with caution.
    /// Feel free to change the storage and behavior that align with your expected business.
    /// The canister should match with the signature defined in `spec.md` in order to be considered as a DIP721 contract.
    #[derive(CandidType, Deserialize, Clone)]
    pub struct TokenMetadata {
        pub token_identifier: TokenIdentifier,
        pub owner: Option<Principal>,
        pub operator: Option<Principal>,
        pub is_burned: bool,
        pub properties: Vec<(String, GenericValue)>,
        pub minted_at: u64,
        pub minted_by: Principal,
        pub transferred_at: Option<u64>,
        pub transferred_by: Option<Principal>,
        pub approved_at: Option<u64>,
        pub approved_by: Option<Principal>,
        pub burned_at: Option<u64>,
        pub burned_by: Option<Principal>,
    }

    #[derive(CandidType, Debug, Deserialize)]
    pub enum NftError {
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
        InvalidOperation
        // Other(String), // for debugging
    }