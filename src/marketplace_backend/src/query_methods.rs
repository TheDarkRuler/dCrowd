use std::collections::HashMap;

use candid::{Nat, Principal};
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use crate::common::guards::caller_is_auth;
use crate::common::structures::{CollectionFullInfo, OwnersDoubleKey};
use crate::memory::{get_owners_nft, get_record_collections};

///
/// Gets the list of canisters assigned to the caller
/// 
/// ## Arguments
/// * `caller` - Optional of caller principal.
///     * if none, then it will return the canisters assigned to the caller of the function
///     * if some, then it will return the canisters assigned to the principal passed as argument
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// 
/// ## Returns
/// * Ok: List of canister assigned to the caller
/// * Error: if the caller does not have any collection or it encouters a problem on transforming the string to pricipal
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_collection_ids(caller: Option<String>, offset: u32, limit: u32) -> Result<Vec<String>, String> {
    let caller = match &caller {
        Some(x) => Principal::from_text(x).expect("Not able to convert string to principal"),
        None => ic_cdk::caller(),
    };

    let res = get_record_collections()
        .iter()
        .filter(|x| (*x.1).owner == caller)
        .map(|x| x.0.to_string())
        .skip(offset as usize)
        .take(limit as usize)
        .collect::<Vec<String>>();
    if res.is_empty() {
        return Err("this caller does not own any collection".to_string());
    }
    Ok(res)
}

///
/// Returns if a collection is still available by checking if the expire date is passed.
/// 
/// ## Arguments
/// * `canister_id` - Canister id of the collection
/// 
/// ## Returns
/// * Ok: true if the collection is still available and false if not 
/// * Error: if the canister id does not exist
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_collection_viability(canister_id: Principal) -> Result<bool, String> {

    let binding = get_record_collections();
    let val = match binding.get(&canister_id) {
        Some(x) => x,
        None => return Err("collection does not exists".to_string())
    };
    Ok(val.expire_date > ic_cdk::api::time())
}

///
/// Returns all canisters of a owner including information about viability and deadlines.
/// 
/// ## Arguments
/// * `caller` - Optional of caller principal.
///     * if none, then it will return the canisters assigned to the caller of the function
///     * if some, then it will return the canisters assigned to the principal passed as argument
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// 
/// ## Returns
/// * Ok: all canisters assigned to a caller, including the availability boolean
/// * Error: if the canister id does not exist
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_all_collections_by_caller(caller: Option<String>, offset: u32, limit: u32) -> Result<Vec<CollectionFullInfo>, String> {
    
    let caller = match &caller {
        Some(x) => Principal::from_text(x).expect("Not able to convert string to principal"),
        None => ic_cdk::caller(),
    };

    let res = get_record_collections()
        .iter()
        .filter(|x| (*x.1).owner == caller)
        .map(|x| CollectionFullInfo { 
            owner: caller, 
            canister_id: *x.0, 
            expire_date: x.1.expire_date, 
            discount_windows: x.1.clone().discount_windows, 
            available: get_collection_viability(*x.0).expect("Error in getting the records from the database"),
            nfts: (*x.1.nfts).to_vec()
        })
        .skip(offset as usize)
        .take(limit as usize)
        .collect::<Vec<CollectionFullInfo>>();
    Ok(res)
}

///
/// Returns all canisters including information about viability and deadlines.
/// 
/// ## Arguments
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// 
/// ## Returns
/// * Ok: all canisters including their availability 
/// * Error: if the canister id does not exist
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_all_collections(offset: u32, limit: u32) -> Result<Vec<CollectionFullInfo>, String> {

    let res = get_record_collections()
        .iter()
        .map(|x| CollectionFullInfo { 
            owner: x.1.owner, 
            canister_id: *x.0, 
            expire_date: x.1.expire_date, 
            discount_windows: x.1.clone().discount_windows, 
            available: get_collection_viability(*x.0).expect("Error in getting the records from the database"),
            nfts: (*x.1.nfts).to_vec()
        })
        .skip(offset as usize)
        .take(limit as usize)
        .collect::<Vec<CollectionFullInfo>>();
    if res.len() == 0 {
        return Err("no collections present".to_string())
    }
    Ok(res)
}

///
/// Returns all canisters including information about viability and deadlines.
/// 
/// ## Arguments
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// 
/// ## Returns
/// * Ok: all canisters including their availability 
/// * Error: if the canister id does not exist
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_all_nfts(offset: u32, limit: u32) -> Result<HashMap<OwnersDoubleKey, Principal>, String> {

    let res = get_owners_nft()
        .iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|x| (*x.0, *x.1))
        .collect::<HashMap<OwnersDoubleKey, Principal>>();
    if res.len() == 0 {
        return Err("no collections present".to_string())
    }
    Ok(res)
}

pub async fn check_balance(owner: String, tkn_id: u128) -> Result<bool, String> {
    let res = ic_cdk::call::<((Principal, Option<Vec<u8>>),), (Result<Nat, _>,)>
        ( MAINNET_LEDGER_CANISTER_ID, "icrc1_balance_of", ((Principal::from_text(owner).expect("unable to transform to principal"), None), ), )
    .await 
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    .0
    .map_err(|e: Nat| format!("ledger transfer error {:?}", e));

    Ok(false)
}

