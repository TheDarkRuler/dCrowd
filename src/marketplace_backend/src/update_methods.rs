use std::str::FromStr;

use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;

use crate::common::structures::{CanisterInfo, MintArg, Errors};
use crate::common::{guards::caller_is_auth, structures::Arg};
use crate::factory::mint_collection_canister;
use crate::memory::insert_record;

///
/// Creates a collection of nft using the ICRC-7 standard and saves in database the principal of the owner of the colletion and the id of the canister collection.
/// Then creates NFTs to the values passed as arguments of different types (Ex: premium, standard, VIP).
///
/// ## Arguments
///     canister_arg: record {
///           icrc7_supply_cap : nat;
///           icrc7_description : opt text;
///           tx_window : opt nat64;
///           icrc7_max_query_batch_size : opt nat;
///           permitted_drift : opt nat64;
///           icrc7_max_take_value : opt nat;
///           icrc7_max_memo_size : opt nat;
///           icrc7_symbol : text;
///           icrc7_max_update_batch_size : opt nat;
///           icrc7_atomic_batch_transfers : opt bool;
///           icrc7_default_take_value : opt nat;
///           icrc7_logo : opt text;
///           icrc7_name : text;
///         };
///     nfts: vec type NftMetadata = record {
///           token_name: text;
///           token_privilege_code: nat8;
///           token_description: text;
///           token_logo: text;
///           quantity: nat;
///         };
///     expire_date: nat64;
///     
///     discount_windows: vec type DiscountWindowArg = record { 
///           expire_date: nat64; 
///           discount_percentage: nat8; 
///         };
///     };
/// 
/// ## Returns
/// * Canister id of the collection
/// * Error
/// 
#[ic_cdk::update(guard = "caller_is_auth")]
pub async fn create_collection_nfts(arg: Arg) -> Result<String, Errors> {

    if arg.expire_date <= ic_cdk::api::time() {
        return Err(Errors::GenericError { 
            message: "Error: Expiration date cannot be in the past".to_string(), 
            error_code: 400
        });
    }

    for x in arg.discount_windows.iter() {
        if (*x).expire_date >= arg.expire_date || (*x).expire_date <= ic_cdk::api::time() {
            return Err(Errors::GenericError { 
                message: "Error: discount windows date cannot be in the past or it cannot be after the expire date".to_string(), 
                error_code: 400
            });
        }
    }

    if arg.nfts.iter().map(|x| x.quantity).sum::<u128>() != arg.canister_arg.icrc7_supply_cap {
        return Err(Errors::GenericError { 
            message: "number of NFTs to create does not match the supply cap".to_string(), 
            error_code: 400
        });
    }

    let canister_id = match mint_collection_canister(arg.canister_arg).await {
        Ok(x) => Principal::from_str(&x).expect("unable to tranform string to Principal"),
        Err(message) => return Err(Errors::GenericError { 
            message, 
            error_code: 400
        }),
    };
    let mut tkn_id = 1;
    let caller = ic_cdk::caller();

    for x in arg.nfts.iter() {
        let mut mint_arg = MintArg {
            to: Account {
                owner: caller,
                subaccount: None,
            },
            memo: None,
            token_id: tkn_id,
            from_subaccount: None,
            token_description: Some((*x).token_description.clone()),
            token_logo: Some((*x).token_logo.clone()),
            token_name: Some((*x).token_name.clone()),
            token_privilege_code: Some((*x).token_privilege_code.clone()),
        };

        for _ in 0..x.quantity {
            
            let (mint_result,): (Result<u128, Errors>,) = ic_cdk::call(canister_id.clone(), "icrc7_mint", (&mint_arg, caller,))
            .await
            .expect("Error in minting NFT");

            if mint_result.is_err() {
                return Err(mint_result.err().expect("error message not loaded"));
            }
            tkn_id += 1;
            mint_arg.token_id = tkn_id;
        }
    }
    insert_record(canister_id, CanisterInfo { owner: caller, expire_date: arg.expire_date, discount_windows: arg.discount_windows });

    Ok(canister_id.to_string())
}