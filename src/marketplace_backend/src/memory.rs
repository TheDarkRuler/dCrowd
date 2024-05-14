use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::common::structures::CanisterInfo;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PRINCIPALS: RefCell<StableBTreeMap<Principal, CanisterInfo, Memory>> = RefCell::new({
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    });
}

pub fn get_records() -> HashMap<Principal, CanisterInfo> {

    PRINCIPALS.with(|x| x.borrow().iter().collect::<HashMap<Principal, CanisterInfo>>())
}

pub fn insert_record(canister: Principal, canister_info: CanisterInfo) {
    
    PRINCIPALS.with(|x| x.borrow_mut().insert(canister, canister_info));
}