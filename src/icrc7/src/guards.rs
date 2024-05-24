use crate::state::STATE;
use candid::Principal;
use dotenv_codegen::dotenv;
use ic_cdk::caller;

pub fn owner_guard() -> Result<(), String> {
    let owner = STATE
        .with(|s| s.borrow().icrc1_minting_authority())
        .ok_or_else(|| String::from("The canister not set owner"))?;

    if caller() == owner.owner {
        Ok(())
    } else {
        Err(String::from("The caller is not the owner of contract"))
    }
}

pub fn authenticated_guard() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() || caller.to_string() != dotenv!("CANISTER_ID_MARKETPLACE_BACKEND") {
        Err("anonymous user is not allowed".to_string())
    } else {
        Ok(())
    }
}
