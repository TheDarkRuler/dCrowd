use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::common::structures::{CollectionInfo, NftMarketData, OwnersDoubleKey};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PRINCIPALS: RefCell<StableBTreeMap<Principal, CollectionInfo, Memory>> = RefCell::new({
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    });

    static OWNERS: RefCell<StableBTreeMap<OwnersDoubleKey, NftMarketData, Memory>> = RefCell::new({
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    });
}

pub fn get_record_collections() -> HashMap<Principal, CollectionInfo> {

    PRINCIPALS.with(|x| x.borrow().iter().collect::<HashMap<Principal, CollectionInfo>>())
}

pub fn insert_record_collection(canister: Principal, collection_info: CollectionInfo) {
    
    PRINCIPALS.with(|x| x.borrow_mut().insert(canister, collection_info));
}

pub fn get_owners_nft() -> HashMap<OwnersDoubleKey, NftMarketData> {

    OWNERS.with(|x| x.borrow().iter().collect::<HashMap<OwnersDoubleKey, NftMarketData>>())
}

pub fn insert_owner_nft(canister: Principal, tkn_id: u64, owner: Principal, price: Option<u32>, on_sale: bool) {
    
    OWNERS.with(|x| 
        x
        .borrow_mut()
        .insert(
            OwnersDoubleKey {collection_id: canister, tkn_id}, 
            NftMarketData {owner, price, on_sale}
        ));
}

