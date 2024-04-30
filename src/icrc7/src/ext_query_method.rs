use candid::Principal;
use ic_cdk::query;

use crate::{
    ext_types::{
        AccountIdentifierHex, ExtAllowanceArg, ExtAllowanceResult, ExtBalanceArg, ExtBalanceResult,
        ExtBearerResult, ExtMetadata, ExtMetadataResult, ExtSupplyResult, ExtTokenIndex, Extension,
        TokenIdentifier, EXTENSIONS,
    },
    state::STATE,
};

#[query(name = "balance")]
pub fn ext_balance(arg: ExtBalanceArg) -> ExtBalanceResult {
    STATE.with(|s| s.borrow().ext_balance(arg))
}

#[query(name = "allowance")]
pub fn ext_allowance(arg: ExtAllowanceArg) -> ExtAllowanceResult {
    STATE.with(|s| s.borrow().ext_allowance(arg))
}

#[query(name = "bearer")]
pub fn ext_bearer(token: TokenIdentifier) -> ExtBearerResult {
    STATE.with(|s| s.borrow().ext_bearer(token))
}

#[query(name = "metadata")]
pub fn ext_metadata(token: TokenIdentifier) -> ExtMetadataResult {
    STATE.with(|s| s.borrow().ext_metadata(token))
}

#[query(name = "getMinter")]
pub fn ext_get_minter() -> Principal {
    let minting_authority = STATE.with(|s| s.borrow().icrc7_minting_authority());
    if let Some(minting_authority_info) = minting_authority {
        minting_authority_info.owner
    } else {
        Principal::anonymous()
    }
}

#[query(name = "getRegistry")]
pub fn ext_get_registry() -> Vec<(ExtTokenIndex, AccountIdentifierHex)> {
    STATE.with(|s| s.borrow().ext_get_registry())
}

#[query(name = "supply")]
pub fn ext_supply(_token: TokenIdentifier) -> ExtSupplyResult {
    STATE.with(|s| s.borrow().ext_supply())
}

#[query(name = "getTokens")]
pub fn ext_get_tokens() -> Vec<(ExtTokenIndex, ExtMetadata)> {
    STATE.with(|s| s.borrow().ext_get_tokens())
}

#[query(name = "getTokensByIds")]
pub fn ext_get_tokens_by_ids(
    token_indexs: Vec<ExtTokenIndex>,
) -> Vec<(ExtTokenIndex, ExtMetadata)> {
    STATE.with(|s| s.borrow().ext_get_tokens_by_ids(token_indexs))
}

#[query(name = "getTokenIdentifier")]
pub fn ext_get_token_identifier(index: u128) -> TokenIdentifier {
    let canister_id = ic_cdk::api::id();
    TokenIdentifier::parse_token_identifier(canister_id, index)
}

#[query(name = "extensions")]
pub fn ext_extensions() -> Vec<Extension> {
    let extensions: Vec<Extension> = EXTENSIONS.iter().map(|&s| s.to_string()).collect();
    extensions
}
