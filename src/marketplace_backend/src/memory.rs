use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use std::collections::HashMap;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PRINCIPALS: RefCell<StableBTreeMap<Principal, Principal, Memory>> = RefCell::new({
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    });
}

pub fn get_records() -> HashMap<Principal, Principal> {

    PRINCIPALS.with(|x| x.borrow().iter().collect::<HashMap<Principal, Principal>>())
}

pub fn insert_record(principal: Principal, canister_id: Principal) {
    
    PRINCIPALS.with(|x| x.borrow_mut().insert(canister_id, principal));
}