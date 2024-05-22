use std::collections::HashMap;

use candid::Principal;
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use icrc_ledger_types::icrc1::account::Account;
use crate::common::guards::caller_is_auth;
use crate::common::structures::{CollectionFullInfo, NftMarketData, OwnersDoubleKey};
use crate::memory::{get_nfts, get_collections};

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
/// * `Ok`: List of canister assigned to the caller
/// * `Error`: if the caller does not have any collection or it encouters a problem on transforming the string to pricipal
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_collection_ids(caller: Option<String>, offset: u32, limit: u32) -> Result<Vec<String>, String> {
    let caller = match &caller {
        Some(x) => Principal::from_text(x).expect("Not able to convert string to principal"),
        None => ic_cdk::caller(),
    };

    let res = get_collections()
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
/// * `Ok`: true if the collection is still available and false if not 
/// * `Error`: if the canister id does not exist
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_collection_viability(canister_id: Principal) -> Result<bool, String> {

    let binding = get_collections();
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
/// * `Ok`: all canisters assigned to a caller, including the availability boolean
/// * `Error`: if the canister id does not exist
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_collections_info_by_caller(caller: Option<String>, offset: u32, limit: u32) -> Result<Vec<CollectionFullInfo>, String> {
    
    let caller = match &caller {
        Some(x) => Principal::from_text(x).expect("Not able to convert string to principal"),
        None => ic_cdk::caller(),
    };

    let res = get_collections()
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
/// * `Ok`: all canisters including their availability 
/// * `Error`: no collection present or error in getting records
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_all_collections(offset: u32, limit: u32) -> Result<Vec<CollectionFullInfo>, String> {

    let res = get_collections()
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
/// Returns all NFTs including their informations.
/// 
/// ## Arguments
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// 
/// ## Returns
/// * `Ok`: all NFTs including their availability 
/// * `Error`: if no nfts can be retrieved, either caused by offset and limit problem or 0 records on the database
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_all_nfts(offset: u32, limit: u32) -> Result<HashMap<OwnersDoubleKey, NftMarketData>, String> {

    let res = get_nfts()
        .iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|x| (*x.0, *x.1))
        .collect::<HashMap<OwnersDoubleKey, NftMarketData>>();
    if res.len() == 0 {
        return Err("no collections present".to_string())
    }
    Ok(res)
}

/// 
/// Function that checks if the balance of an account is enough to purchase an NFT passed.
/// 
/// ## Arguments
/// * `owner` - Offset of the first element to retrieve
/// * `tkn_id` - Number of elements to retrieve
/// * `collection_id` - id of the collection canister
/// 
/// ## Returns
/// * `Ok`: Full price of the nft
/// * `Error`: Error string
/// 
#[ic_cdk::query(guard = "caller_is_auth", composite = true)]
pub async fn check_balance(owner: Option<String>, tkn_id: u64, collection_id: String) -> Result<u128, String> {

    let owner = match owner {
        Some(x) => Principal::from_text(x).expect("unable to parse owner to principal"),
        None => ic_cdk::caller(),
    }; 

    let price = match get_nfts()
        .get(&OwnersDoubleKey { 
            collection_id: Principal::from_text(&collection_id).expect("cannot convert from text to principal"), 
            tkn_id 
        }) {
        
        Some(&x) => {
            if !x.on_sale {
                return Err("NFT not on sale".to_string());
            }
            match x.price {
                Some(price) => {
                    get_discount(price, collection_id, x.owner)
                },
                None => Err("NFT price not present".to_string()),
            }
        },
        None => Err("Nft does not exists".to_string()),        
    }?;

    let balance = ic_cdk::call::<(Account,), (u128,)>
        ( MAINNET_LEDGER_CANISTER_ID, "icrc1_balance_of", (Account::from(owner),) )
        .await 
        .map_err(|e| format!("failed to call ledger: {:?}", e))?.0;

    let fee = ic_cdk::call::<(), (u128,)>
        ( MAINNET_LEDGER_CANISTER_ID, "icrc1_fee", () )
        .await 
        .map_err(|e| format!("failed to call ledger: {:?}", e))?.0;

    if (price + fee) <= balance {
        return Ok(price + fee)
    }
    Err("Low balance".to_string())

}


/// 
/// Function that checks if the collection assigned to the NFT is expired or if it is owned by another person and returnes the price either discounted or not 
/// based on the discount windows of the collection.
/// 
/// ## Arguments
/// * `price` - default price of the NFT
/// * `collection_id` - id of the collection canister
/// * `owner` - owner of the NFT
/// 
/// ## Returns
/// * `Ok`: price either discounted or not, based on the ownage of the NFT and the discount windows
/// * `Error`: collection expired or parsing errors
/// 
fn get_discount(price: u32, collection_id: String, owner: Principal) -> Result<u128, String> {

    let now = ic_cdk::api::time();
    let price = price as u128;
    let binding = get_collections();
    let collection_info = binding
        .get(&Principal::from_text(collection_id).expect("unable to parse collection id to pricipal"))
        .expect("collection does not exists");

    if owner != collection_info.owner {
        return Ok(price)
    } else if now > collection_info.expire_date {
        return Err("collection Expired".to_string());
    }

    match collection_info.discount_windows
        .iter()
        .filter(|x| x.expire_date > now)
        .min_by_key(|x| x.expire_date) {

        Some(x) => Ok(price - ((price * (x.discount_percentage as u128)) / 100)),
        None => Ok(price),
    }
}

