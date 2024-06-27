use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};
use std::cell::RefCell;
mod constants;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub enum MemoryLocation {
    Exchanges,
    UserKeys,
    UserRequest,
}

impl MemoryLocation {
    pub fn memory_id(self) -> MemoryId {
        match self {
            MemoryLocation::Exchanges => constants::EXCHANGE_STORE_MEMORY_ID,
            MemoryLocation::UserKeys => constants::USER_KEYS_STORE_MEMORY_ID,
            MemoryLocation::UserRequest => constants::USER_REQUESTS_STORE_MEMORY_ID,
        }
    }
}
