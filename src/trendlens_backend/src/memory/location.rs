use super::constants;
use ic_stable_structures::memory_manager::MemoryId;

pub enum MemoryLocation {
    Exchanges,
    UserKeys,
    ApiKeys,
    UserRequest,
}

impl MemoryLocation {
    pub fn memory_id(self) -> MemoryId {
        match self {
            MemoryLocation::Exchanges => constants::EXCHANGE_TABLE_MEMORY_ID,
            MemoryLocation::UserKeys => constants::USER_KEYS_TABLE_MEMORY_ID,
            MemoryLocation::UserRequest => constants::USER_REQUESTS_TABLE_MEMORY_ID,
            MemoryLocation::ApiKeys => constants::API_KEYS_TABLE_MEMORY_ID,
        }
    }
}
