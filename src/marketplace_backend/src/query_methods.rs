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
/// * List of canister assigned to the caller
/// 
#[ic_cdk::query(guard = "caller_is_auth")]
pub fn get_canister_ids(caller: Option<String>) -> Result<Vec<String>, String> {
    let caller = match &caller {
        Some(x) => Principal::from_text(x).expect("not able to convert string to principal"),
        None => ic_cdk::caller(),
    };
    
    let res = get_records().iter().filter(|x| *x.1 == caller).map(|x| x.0.to_string()).collect::<Vec<String>>();
    if res.is_empty() {
        return Err("this caller does not own any collection".to_string());
    }
    Ok(res)
}