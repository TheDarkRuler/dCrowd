use std::str::FromStr;

use candid::Principal;
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::BlockIndex;
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};

use crate::common::structures::{CollectionInfo, CollectionNfts, Errors, MintArg, TransferArgs};
use crate::common::{guards::caller_is_auth, structures::Arg};
use crate::factory::mint_collection_canister;
use crate::memory::{insert_owner_nft, insert_record_collection};

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
/// * Ok: Canister id of the collection
/// * Error: Error of type Errors
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

    if arg.nfts.iter().map(|x| x.quantity).sum::<u64>() as u128 != arg.canister_arg.icrc7_supply_cap {
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

    let mut nfts: Vec<CollectionNfts> = Vec::new();

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
        let mut tkn_ids:Vec<u64> = Vec::new();
        for _ in 0..x.quantity {

            tkn_ids.push(tkn_id as u64);

            let (mint_result,): (Result<u128, Errors>,) = ic_cdk::call(canister_id.clone(), "icrc7_mint", (&mint_arg, caller,))
            .await
            .expect("Error in minting NFT");

            if mint_result.is_err() {
                return Err(mint_result.err().expect("error message not loaded"));
            }
            insert_owner_nft(canister_id, tkn_id as u64, caller, Some(x.price), true);
            tkn_id += 1;
            mint_arg.token_id = tkn_id;
        }
        nfts.push(CollectionNfts {nft: x.clone(), tkn_ids});
    }
    insert_record_collection(canister_id, CollectionInfo { owner: caller, expire_date: arg.expire_date, discount_windows: arg.discount_windows, nfts});

    Ok(canister_id.to_string())
}

///
/// Transfer amount of tokens from an account to another,
/// before calling this function it is needed to approve the tokens to transfer + the transaction fee to this backend canister
/// so that this canister can have the allowance to transfer tokens in behalf of the caller
///
/// ## Arguments
///     type TransferArgs = record { 
///         to_account : Account; 
///         amount : nat; 
///     };
/// 
/// ## Returns
/// * Ok: Transaction id
/// * Error: String with some details about what went wrong
/// 
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

    let transfer_from_args = TransferFromArgs {
        from: Account::from(ic_cdk::caller()),
        memo: None,
        amount: args.amount,
        spender_subaccount: None,
        fee: None,
        to: args.to_account,
        created_at_time: None,
    };

    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>
    ( MAINNET_LEDGER_CANISTER_ID, "icrc2_transfer_from", (transfer_from_args,), )
    .await 
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    .0
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

/// dfx deploy --specified-id ryjl3-tyaaa-aaaaa-aaaba-cai icp_ledger_canister --argument "
///   (variant {
///     Init = record {
///       minting_account = \"$MINTER_ACCOUNT_ID\";
///       initial_values = vec {
///         record {
///           \"$DEFAULT_ACCOUNT_ID\";
///           record {
///             e8s = 10_000_000_000 : nat64;
///           };
///         };
///       };
///       send_whitelist = vec {};
///       transfer_fee = opt record {
///         e8s = 10_000 : nat64;
///       };
///       token_symbol = opt \"LICP\";
///       token_name = opt \"Local ICP\";
///     }
///   })
/// "
#[ic_cdk::update(guard = "caller_is_auth")]
pub async fn transfer_nft(args: TransferArgs) -> Result<String, String> {
    match transfer(args).await {
        Ok(_) => Ok("token transfered correctly".to_string()),
        Err(e) => Err(format!("error in transfering tokens: {}", e)),
    }
}