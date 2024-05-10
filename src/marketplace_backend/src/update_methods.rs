use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;

use crate::common::structures::{MintArg, MintError};
use crate::common::{guards::caller_is_auth, structures::Arg};
use crate::factory::mint_collection_canister;

///
/// Creates a collection of nft using the ICRC-7 standard and saves in database the principal of the owner of the colletion and the id of the canister collection.
/// Then creates NFTs to the values passed as arguments of different types (Ex: premium, standard, VIP).
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
pub async fn create_collection_nfts(arg: Arg) -> Result<String, MintError> {
    
    if arg.nfts.iter().map(|x| x.quantity).sum::<u128>() != arg.canister_arg.icrc7_supply_cap {
        return Err(MintError::GenericError { 
            message: "number of NFTs to create does not match the supply cap".to_string(), 
            error_code: 400
        });
    }

    let canister_id = match mint_collection_canister(arg.canister_arg).await {
        Ok(x) => x,
        Err(message) => return Err(MintError::GenericError { 
            message, 
            error_code: 400
        }),
    };
    
    let mut tkn_id = 0;
    for x in arg.nfts.iter() {
        let mint_arg = MintArg {
            to: Account {
                owner: ic_cdk::caller(),
                subaccount: None,
            },
            token_id: tkn_id,
            from_subaccount: None,
            token_description: Some(x.description.clone()),
            token_logo: Some(x.logo.clone()),
            token_name: Some(x.name.clone()),
            token_privilege_code: Some(x.privilege_code.clone()),
        };

        let (mint_result,): (Result<u128, MintError>,) = ic_cdk::call(Principal::from_text(canister_id.clone())
            .expect("unable to convert string to canister id"), "icrc7_mint", (mint_arg,))
            .await
            .expect("Error in minting NFT");

        if mint_result.is_err() {
            return Err(mint_result.err().unwrap());
        }

        tkn_id += 1;
    }
    Ok(canister_id)
}