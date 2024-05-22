use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::common::structures::{CollectionInfo, NftMarketData, OwnersDoubleKey};

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Creation of the datasets stored on the stable memory of ICP
thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static COLLECTIONS: RefCell<StableBTreeMap<Principal, CollectionInfo, Memory>> = RefCell::new({
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    });

    static NFTS: RefCell<StableBTreeMap<OwnersDoubleKey, NftMarketData, Memory>> = RefCell::new({
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    });
}

///
/// Gets hashmap of collection ids as keys and collections info as values
/// 
/// ## Returns
/// * HashMap of collection ids as key and collection info as values
/// 
/// ## Structures
/// ``` rust
/// pub struct CollectionInfo {
///     pub owner: Principal,
///     pub expire_date: u64,
///     pub discount_windows: Vec<DiscountWindowArg>,
///     pub nfts: Vec<CollectionNfts>,
/// }
/// ```
/// 
pub fn get_collections() -> HashMap<Principal, CollectionInfo> {

    COLLECTIONS.with(|x| x.borrow().iter().collect::<HashMap<Principal, CollectionInfo>>())
}

///
/// Inserts a collection and his info on stable memory of ICP
/// 
/// ## Arguments
/// * `canister` - collection canister id as principal
/// * `collection_info` - infos of the collection
/// 
/// 
/// # Structures
/// ``` rust
/// pub struct CollectionInfo {
///     pub owner: Principal,
///     pub expire_date: u64,
///     pub discount_windows: Vec<DiscountWindowArg>,
///     pub nfts: Vec<CollectionNfts>,
/// }
/// ```
/// 
pub fn insert_collection_record(canister: Principal, collection_info: CollectionInfo) {
    
    COLLECTIONS.with(|x| x.borrow_mut().insert(canister, collection_info));
}

///
/// Gets hashmap of a pair of collection_id and nft_id as keys and NftMarkeData as values
/// 
/// ## Returns
/// * HashMap<OwnersDoubleKey, NftMarketData> 
/// 
/// ## Structures
/// ``` rust
/// pub struct OwnersDoubleKey {
///     pub collection_id: Principal,
///     pub tkn_id: u64
/// }
/// 
/// pub struct NftMarketData {
///     pub owner: Principal,
///     pub price: Option<u32>,
///     pub on_sale: bool
/// }
/// ```
/// 
pub fn get_nfts() -> HashMap<OwnersDoubleKey, NftMarketData> {

    NFTS.with(|x| x.borrow().iter().collect::<HashMap<OwnersDoubleKey, NftMarketData>>())
}

///
/// Inserts a an NFT on the database of the marketplace, using a pair of collection_id and nft_id as key and NftMarketData as value
/// 
/// ## Arguments
/// * `canister` - collection canister id as principal
/// * `tkn_id` - id of the token
/// * `owner` - owner of the NFT
/// * `price` - Optional of price of the NFT 
/// * `on_sale` - boolean 
/// 
pub fn insert_nft_record(canister: Principal, tkn_id: u64, owner: Principal, price: Option<u32>, on_sale: bool) {
    
    NFTS.with(|x| 
        x
        .borrow_mut()
        .insert(
            OwnersDoubleKey {collection_id: canister, tkn_id}, 
            NftMarketData {owner, price, on_sale}
        ));
}

