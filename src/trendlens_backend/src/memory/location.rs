use ic_stable_structures::memory_manager::MemoryId;
use super::constants;

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
