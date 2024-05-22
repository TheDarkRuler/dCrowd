use candid::Principal;

///
/// Checks if the caller is Anonymous
/// 
/// ## Returns
/// * `Ok` - the caller is authenticated
/// * `Err` - the caller is Anonymous
/// 
pub fn caller_is_auth() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err("Anonymous Caller".into());
    }
    Ok(())
}
