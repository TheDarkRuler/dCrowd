use candid::Principal;
use crate::common::guards::caller_is_auth;
use crate::memory::get_records;

///
/// Gets the list of canisters assigned to the caller
/// 
/// ## Arguments
/// * Optional of caller principal.
///     * if none, then it will return the canisters assigned to the caller of the function
///     * if some, then it will return the canisters assigned to the principal passed as argument
/// 
/// ## Returns
/// * Ok: List of canister assigned to the caller
/// * Error: if the caller does not have any collection or it encouters a problem on transforming the string to pricipal
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_canister_ids(caller: Option<String>) -> Result<Vec<String>, String> {
    let caller = match &caller {
        Some(x) => Principal::from_text(x).expect("Not able to convert string to principal"),
        None => ic_cdk::caller(),
    };

    let res = get_records().iter().filter(|x| (*x.1).owner == caller).map(|x| x.0.to_string()).collect::<Vec<String>>();
    if res.is_empty() {
        return Err("this caller does not own any collection".to_string());
    }
    Ok(res)
}

///
/// Returns if a collection is still available by checking if the expire date is passed.
/// 
/// ## Arguments
/// * Canister id of the collection
/// 
/// ## Returns
/// * Ok: true if the collection is still available and false if not 
/// * Error: if the canister id does not exist
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn canister_viability(canister_id: Principal) -> Result<bool, String> {

    let binding = get_records();
    let val = match binding.get(&canister_id) {
        Some(x) => x,
        None => return Err("collection does not exists".to_string())
    };
    Ok(val.expire_date > ic_cdk::api::time())
}

