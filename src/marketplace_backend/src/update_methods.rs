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
pub async fn create_collection_nfts(arg: Arg) -> Result<String, String> {
    
    let canister_id = mint_collection_canister(arg.canister_arg)
        .await?;
    Ok(canister_id)
}