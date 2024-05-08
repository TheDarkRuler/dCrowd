use crate::common::guards::caller_is_auth;
use crate::common::structures::InitArg;
use crate::common::structures::Arg;
use crate::memory::insert_record;
use candid::Encode;
use ic_cdk::api::management_canister::{
    main::{create_canister, install_code, CreateCanisterArgument, InstallCodeArgument},
    provisional::CanisterSettings,
};
use icrc_ledger_types::icrc1::account::Account;

pub const ICRC7_WASM: &[u8] = std::include_bytes!("../../../wasm_files/icrc7.wasm");


///
/// Creates a collection of nft using the ICRC-7 standard and saves in database the principal of the owner of the colletion and the id of the canister collection.
///
/// ## Arguments
/// *   icrc7_supply_cap : opt nat;
/// *   icrc7_description : opt text;
/// *   tx_window : opt nat64;
/// *   icrc7_max_query_batch_size : opt nat;
/// *   permitted_drift : opt nat64;
/// *   icrc7_max_take_value : opt nat;
/// *   icrc7_max_memo_size : opt nat;
/// *   icrc7_symbol : text;
/// *   icrc7_max_update_batch_size : opt nat;
/// *   icrc7_atomic_batch_transfers : opt bool;
/// *   icrc7_default_take_value : opt nat;
/// *   icrc7_logo : opt text;
/// *   icrc7_name : text;
///
/// ## Returns
/// * canister id of the collection
/// 
#[ic_cdk::update(guard = "caller_is_auth")]
async fn mint_collection_canister(arg: Arg) -> Result<String, String> {
    let caller = ic_cdk::caller();
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
