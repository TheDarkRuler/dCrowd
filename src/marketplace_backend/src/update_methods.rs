use std::str::FromStr;
use candid::Principal;
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::BlockIndex;
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};

use crate::common::structures::{CollectionInfo, CollectionNfts, Errors, IcrcTransferArg, MintArg, OwnersDoubleKey, TransferArgs, TransferError};
use crate::common::{guards::caller_is_auth, structures::Arg};
use crate::factory::mint_collection_canister;
use crate::memory::{get_nfts, insert_nft_record, insert_collection_record};

///
/// Creates a collection of nft using the ICRC-7 standard and saves in database the principal of the owner of the colletion and the id of the canister collection.
/// Then creates NFTs to the values passed as arguments of different types (Ex: premium, standard, VIP).
///
/// ## Arguments
/// * `arg`: 
/// ```
///     pub struct Arg {
///         pub canister_arg: CanisterArg,
///         pub nfts: Vec<NftMetadata>,
///         pub expire_date: u64,
///         pub discount_windows: Vec<DiscountWindowArg>
///     }
/// 
/// ```
/// ## Returns
/// * `Ok`: Successful message
/// * `Error`: Error of type Errors
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
        if x.expire_date >= arg.expire_date || x.expire_date <= ic_cdk::api::time() {
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
            token_description: Some(x.token_description.clone()),
            token_logo: Some(x.token_logo.clone()),
            token_name: Some(x.token_name.clone()),
            token_privilege_code: Some(x.token_privilege_code),
        };
        let mut tkn_ids:Vec<u64> = Vec::new();
        for _ in 0..x.quantity {

            tkn_ids.push(tkn_id as u64);

            let (mint_result,): (Result<u128, Errors>,) = ic_cdk::call(canister_id, "icrc7_mint", (&mint_arg, caller,))
            .await
            .expect("Error in minting NFT");

            if mint_result.is_err() {
                return Err(mint_result.expect_err("error message not loaded"));
            }
            insert_nft_record(canister_id, tkn_id as u64, caller, Some(x.price), true);
            tkn_id += 1;
            mint_arg.token_id = tkn_id;
        }
        nfts.push(CollectionNfts {nft: x.clone(), tkn_ids});
    }
    insert_collection_record(canister_id, CollectionInfo { owner: caller, expire_date: arg.expire_date, discount_windows: arg.discount_windows, nfts});

    Ok(canister_id.to_string())
}

///
/// Transfer amount of tokens from an account to another,
/// before calling this function it is needed to approve the tokens to transfer + the transaction fee to this backend canister
/// so that this canister can have the allowance to transfer tokens in behalf of the caller
///
/// ## Arguments
/// * `args`: 
/// ```
///     type TransferArgs = record { 
///       amount : nat; 
///       tkn_id : nat;
///       collection_id : text;
///     };
/// ```
/// * caller: account to which the nft will be transferred
/// 
/// ## Returns
/// * `Ok`: Transaction id
/// * `Error`: String with some details about what went wrong
/// 
async fn transfer(args: TransferArgs, owner_nft: Principal) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &owner_nft,
    );

    let transfer_from_args = TransferFromArgs {
        from: Account::from(ic_cdk::caller()),
        memo: None,
        amount: args.amount,
        spender_subaccount: None,
        fee: None,
        to: Account::from(owner_nft),
        created_at_time: None,
    };

    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>
        ( MAINNET_LEDGER_CANISTER_ID, "icrc2_transfer_from", (transfer_from_args,), )
            .await 
            .map_err(|e| format!("failed to call ledger: {:?}", e))?
            .0
            .map_err(|e| format!("ledger transfer error {:?}", e))
}

///
/// Transfer NFT from an account to another,
///
/// ## Arguments
/// * `args`:
///     type TransferArgs = record { 
///       amount : nat; 
///       tkn_id : nat;
///       collection_id : text;
///     };
/// 
/// ## Returns
/// * `Ok`: Successful message
/// * `Error`: String with some details about what went wrong
/// 
#[ic_cdk::update(guard = "caller_is_auth")]
pub async fn transfer_nft(args: TransferArgs) -> Result<String, String> {
    let collection_id = Principal::from_text(args.clone().collection_id).expect("unable to parse string to principal");

    let binding = get_nfts();
    let owner_nft = binding
        .get(
            &OwnersDoubleKey {
                collection_id: Principal::from_text(args.clone().collection_id).expect("unable to convert collection id to principal"), 
                tkn_id: args.tkn_id as u64
            });

    if owner_nft.is_none() {
        return Err("nft does not exists".to_string());
    }

    let owner_nft = owner_nft.unwrap().owner;
    let caller = ic_cdk::caller();
    
    let transfer_token = transfer(args.clone(), owner_nft).await;
    if let Some(e) = transfer_token.err() {
        return Err(format!("Error in transfering tokens: {}", e))
    }

    let transfer_nft: Result<u128, TransferError> = match ic_cdk::call::<(Vec<IcrcTransferArg>, Option<Principal> ), (Vec<Option<Result<u128, TransferError>>>,)>(
        collection_id, 
        "icrc7_transfer", 
        ([IcrcTransferArg {
            from_subaccount: None, 
            to: Account::from(caller), 
            token_id: args.tkn_id, 
            memo: None, 
            created_at_time: None
        }].to_vec(), Some(owner_nft), ), )
    .await
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    .0.first() {
        Some(Some(trasfer_el)) => {
            trasfer_el.clone()
        },
        _ => Err(TransferError::GenericError { error_code: 400, message: "error in transfering NFT".to_string() })
    };

    match transfer_nft {
        Ok(_) => {
            insert_nft_record(collection_id, args.tkn_id as u64, caller, None, false);
            Ok(format!("NFT with token id: {}, transferred from {} to {} correctly", args.tkn_id, owner_nft, caller))
        },
        Err(e) => Err(format!("Error in transfering NFT {:?}", e)),                
    }
}