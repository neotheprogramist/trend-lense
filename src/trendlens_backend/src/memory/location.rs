use super::constants;
use ic_stable_structures::memory_manager::MemoryId;

pub enum MemoryLocation {
    Exchanges,
    UserKeys,
    ApiKeys,
    Instructions,
    Transactions,
    UserTransactions,
    ExchangeInstruments,
}

impl MemoryLocation {
    pub fn memory_id(self) -> MemoryId {
        match self {
            MemoryLocation::Exchanges => constants::EXCHANGE_TABLE_MEMORY_ID,
            MemoryLocation::UserKeys => constants::USER_KEYS_TABLE_MEMORY_ID,
            MemoryLocation::ApiKeys => constants::API_KEYS_TABLE_MEMORY_ID,
            MemoryLocation::Instructions => constants::INSTRUCTIONS_TABLE_MEMORY_ID,
            MemoryLocation::Transactions => constants::TRANSACTIONS_TABLE_MEMORY_ID,
            MemoryLocation::UserTransactions => constants::USER_TRANSACTIONS_TABLE_MEMORY_ID,
            MemoryLocation::ExchangeInstruments => constants::EXCHANGE_INSTRUMENTS_TABLE_MEMORY_ID,
        }
    }
}
