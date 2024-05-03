pub mod common;
pub mod memory;

use crate::common::structures::InitArg;
use crate::common::structures::Arg;
use candid::{Encode, Principal};
use ic_cdk::api::management_canister::{
    main::{create_canister, install_code, CreateCanisterArgument, InstallCodeArgument},
    provisional::CanisterSettings,
};
use icrc_ledger_types::icrc1::account::Account;
use memory::get_records;
use memory::insert_record;

pub const ICRC7_WASM: &[u8] = std::include_bytes!("../../../wasm_files/icrc7.wasm");

#[ic_cdk::update]
async fn mint_collection_canister(arg: Arg) -> Result<String, String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err("Anonymous Caller".into());
    }
    let account = Account {
        owner: caller.clone(),
        subaccount: None,
    };
    let principal = match create_canister(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(vec![ic_cdk::id(), caller.clone()]),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
                reserved_cycles_limit: None
            }),
        },
        1_000_000_000_000,
    )
    .await
    {
        Err((code, msg)) => return Err(format!("Rejection Code: {:?}, Message: {:?}", code, msg)),
        Ok((principal,)) => principal.canister_id,
    };
    let init_arg = InitArg::from((account, arg));
    let init_arg = Encode!(&init_arg).unwrap();
    match install_code(InstallCodeArgument {
        mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Install,
        canister_id: principal,
        wasm_module: ICRC7_WASM.to_vec(),
        arg: init_arg,
    })
    .await
    {
        Ok(()) => {
            insert_record(caller, principal);
            Ok(principal.to_string())
        },
        Err((code, msg)) => Err(format!("Code: {:?}, Message: {:?}", code, msg)),
    }
}

#[ic_cdk::query]
fn get_principal() -> Result<Vec<String>, String> {
    let caller: Principal = ic_cdk::caller();
    let res = get_records().iter().filter(|x| *x.1 == caller).map(|x| x.0.to_string()).collect::<Vec<String>>();
    if res.is_empty() {
        return Err("this caller does not own any collection".to_string());
    }
    Ok(res)
}
