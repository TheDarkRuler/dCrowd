use std::borrow::Cow;

use candid::{CandidType, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use icrc_ledger_types::icrc1::account::Account;

///
/// Init args of the collection of NFTs
/// 
#[derive(CandidType, Serialize)]
pub struct InitArg {
    pub minting_account: Option<Account>,
    pub icrc7_symbol: String,
    pub icrc7_name: String,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_max_query_batch_size: Option<u128>,
    pub icrc7_max_update_batch_size: Option<u128>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u128>,
    pub icrc7_atomic_batch_transfers: Option<bool>,
    pub tx_window: Option<u64>,
    pub permitted_drift: Option<u64>,
}

///
/// Short version of InitArg
/// 
#[derive(CandidType, Deserialize)]
pub struct CanisterArg {
    pub icrc7_symbol: String,
    pub icrc7_name: String,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_supply_cap: u128,
    pub icrc7_max_query_batch_size: Option<u128>,
    pub icrc7_max_update_batch_size: Option<u128>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u128>,
    pub icrc7_atomic_batch_transfers: Option<bool>,
    pub tx_window: Option<u64>,
    pub permitted_drift: Option<u64>,
}

impl From<(Account, CanisterArg)> for InitArg {
    fn from((account, arg): (Account, CanisterArg)) -> Self {
        Self {
            minting_account: Some(account),
            icrc7_symbol: arg.icrc7_symbol,
            icrc7_name: arg.icrc7_name,
            icrc7_description: arg.icrc7_description,
            icrc7_logo: arg.icrc7_logo,
            icrc7_supply_cap: Some(arg.icrc7_supply_cap),
            icrc7_max_query_batch_size: arg.icrc7_max_query_batch_size,
            icrc7_max_update_batch_size: arg.icrc7_max_update_batch_size,
            icrc7_max_take_value: arg.icrc7_max_take_value,
            icrc7_default_take_value: arg.icrc7_default_take_value,
            icrc7_max_memo_size: arg.icrc7_max_memo_size,
            icrc7_atomic_batch_transfers: arg.icrc7_atomic_batch_transfers,
            tx_window: arg.tx_window,
            permitted_drift: arg.permitted_drift,
        }
    }
}

///
/// Metadata of a NFT including the quantity of that single NFT 
/// Quantity is the number of nfts of that type that are going to be created
/// 
#[derive(CandidType, Deserialize, Debug, Serialize, Clone)]
pub struct NftMetadata {
    pub token_name: String,
    pub token_privilege_code: u8,
    pub token_description: String,
    pub quantity: u64,
    pub token_logo: String,
    pub price: u32
}

#[derive(CandidType, Deserialize, Debug, Serialize, Clone)]
pub struct CollectionNfts {
    pub nft: NftMetadata,
    pub tkn_ids: Vec<u64>
}

/// 
/// Marketplace data of the NFT
/// 
#[derive(CandidType, Deserialize, Debug, Serialize, Clone, Copy)]
pub struct NftMarketData {
    pub owner: Principal,
    pub price: Option<u32>,
    pub on_sale: bool
}

impl Storable for NftMarketData {
    fn to_bytes(&self) -> Cow<[u8]> { 

        Cow::Owned(serde_json::to_string(self).expect("failed to serialize to bytes").as_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {

        serde_json::from_str(String::from_utf8(bytes.to_vec()).expect("failed to serialize from bytes").as_str())
            .expect("failed to serialize from bytes")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 2048,
        is_fixed_size: false,
    };
}

///
/// Pair used as key for the NFT StableBTree
/// 
#[derive(CandidType, Deserialize, Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct OwnersDoubleKey {
    pub collection_id: Principal,
    pub tkn_id: u64
}

impl Storable for OwnersDoubleKey {

    fn to_bytes(&self) -> Cow<[u8]> { 

        Cow::Owned(serde_json::to_string(self).expect("failed to serialize to bytes").as_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {

        serde_json::from_str(String::from_utf8(bytes.to_vec()).expect("failed to serialize from bytes").as_str())
            .expect("failed to serialize from bytes")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1024,
        is_fixed_size: false,
    };
}

///
/// Discount window including the expire date of the window and the percentage of sale
/// 
#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct DiscountWindowArg {
    pub expire_date: u64,
    pub discount_percentage: u8 
}

///
/// General arg passed to create a collection of NFTs
/// 
#[derive(CandidType, Deserialize)]
pub struct Arg {
    pub canister_arg: CanisterArg,
    pub nfts: Vec<NftMetadata>,
    pub expire_date: u64,
    pub discount_windows: Vec<DiscountWindowArg>
}

///
/// Args passed to the method icrc7_mint
/// 
#[derive(CandidType, Deserialize)]
pub struct MintArg {
    pub to : Account,
    pub token_id : u128,
    pub memo: Option<Vec<u8>>,
    pub from_subaccount : Option<Vec<u8>>,
    pub token_description : Option<String>,
    pub token_logo : Option<String>,
    pub token_name : Option<String>,
    pub token_privilege_code: Option<u8>
  }

  ///
  /// Types of possible Errors the project can return
  /// 
#[derive(CandidType, Deserialize, Debug)]
pub enum Errors {
    GenericError { message : String, error_code : u128 },
    SupplyCapReached,
    TokenIdMinimumLimit,
    Unauthorized,
    GenericBatchError { message : String, error_code : u128 },
    TokenIdAlreadyExist,
}

///
/// Full information about a collection
/// 
#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct CollectionFullInfo {
    pub owner: Principal,
    pub canister_id: Principal,
    pub expire_date: u64,
    pub discount_windows: Vec<DiscountWindowArg>,
    pub available: bool,
    pub nfts: Vec<CollectionNfts>
}

///
/// Part of indormations about a collection
/// 
#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct CollectionInfo {
    pub owner: Principal,
    pub expire_date: u64,
    pub discount_windows: Vec<DiscountWindowArg>,
    pub nfts: Vec<CollectionNfts>
}

impl Storable for CollectionInfo {
    fn to_bytes(&self) -> Cow<[u8]> { 

        Cow::Owned(serde_json::to_string(self).expect("failed to serialize to bytes").as_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {

        serde_json::from_str(String::from_utf8(bytes.to_vec()).expect("failed to serialize from bytes").as_str())
            .expect("failed to serialize from bytes")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 2048,
        is_fixed_size: false,
    };
}

///
/// Args for transfering tokens
/// 
#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct TransferArgs {
    pub tkn_id: u128,
    pub collection_id: String
}

///
/// Possible Errors of the transfer method
/// 
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum TransferError {
    NonExistingTokenId,
    InvalidRecipient,
    Unauthorized,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u128 },
    GenericError { error_code: u128, message: String },
    GenericBatchError { error_code: u128, message: String },
}

///
/// Transfer args for transfering NFT
/// 
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcrcTransferArg {
    pub from_subaccount: Option<[u8; 32]>,
    pub to: Account,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}