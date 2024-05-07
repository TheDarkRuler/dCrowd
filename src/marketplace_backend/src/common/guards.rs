use candid::Principal;

pub fn caller_is_auth() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err("Anonymous Caller".into());
    }
    Ok(())
}
