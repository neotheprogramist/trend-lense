use crate::{
    exchange::Exchange,
    memory::{Memory, MemoryLocation, MEMORY_MANAGER},
    storable_wrapper::StorableWrapper,
};
use candid::{CandidType, Principal};
use ic_stable_structures::StableBTreeMap;
use request::Request;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, ops::Deref};

pub mod request;

#[derive(Debug, Deserialize, Serialize, Clone, CandidType)]
pub struct Instruction {
    pub exchange: Exchange,
    pub api_key: String,
    pub request: Request,
}

#[derive(Debug, Deserialize, Serialize, Clone, CandidType)]
pub struct Transaction(pub Vec<Instruction>);

impl Deref for Transaction {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// clone right now 
impl Transaction {
    pub fn get_instruction(&self, index: usize) -> Option<Instruction> {
        self.get(index).cloned()
    }
}

type InstructionId = u32;
type InstructionsTable = StableBTreeMap<InstructionId, StorableWrapper<Instruction>, Memory>;
type TransactionId = u32;
type TransactionsTable = StableBTreeMap<TransactionId, StorableWrapper<Vec<InstructionId>>, Memory>;
type UserTransactionsTable = StableBTreeMap<Principal, StorableWrapper<Vec<TransactionId>>, Memory>;

thread_local! {
    static INSTRUCTIONS: RefCell<InstructionsTable> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::Instructions.memory_id())),
        )
    );

    static TRANSACTIONS: RefCell<TransactionsTable> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::Transactions.memory_id())),
        )
    );

    static USER_TRANSACTIONS: RefCell<UserTransactionsTable> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::UserTransactions.memory_id())),
        )
    );
}

// consider making users and then implementing it as trait not using struct but directly
// use getters to extract useful information from contract state
pub struct TransactionStore;

impl TransactionStore {
    fn insert_instruction(instruction: Instruction) -> InstructionId {
        INSTRUCTIONS.with_borrow_mut(|k| {
            let instruction_id = k.len() as InstructionId;
            k.insert(instruction_id, StorableWrapper(instruction));
            instruction_id
        })
    }

    fn insert_transaction(instructions: Vec<Instruction>) -> TransactionId {
        TRANSACTIONS.with_borrow_mut(|k| {
            let transaction_id = k.len() as TransactionId;

            let instructions_ids: Vec<InstructionId> = instructions
                .into_iter()
                .map(|i| Self::insert_instruction(i))
                .collect();

            k.insert(transaction_id, StorableWrapper(instructions_ids));

            transaction_id
        })
    }

    pub fn add_transaction(identity: &Principal, instructions: Vec<Instruction>) -> TransactionId {
        let transaction_id = Self::insert_transaction(instructions);

        USER_TRANSACTIONS.with_borrow_mut(|k| {
            let mut before = k.get(identity).unwrap_or_default();
            before.push(transaction_id);
            k.insert(*identity, before);
        });

        transaction_id
    }

    pub fn get_transaction(
        identity: &Principal,
        transaction_id: TransactionId,
    ) -> Option<Transaction> {
        let has_transaction =
            USER_TRANSACTIONS.with_borrow(|k| Some(k.get(identity)?.contains(&transaction_id)))?;

        if !has_transaction {
            return None;
        }

        TRANSACTIONS.with_borrow(|k| {
            let instructions_ids = k.get(&transaction_id)?;

            let instructions = instructions_ids
                .iter()
                .filter_map(|i| Some(INSTRUCTIONS.with_borrow(|k| k.get(&i))?.0))
                .collect::<Vec<_>>();

            Some(Transaction(instructions))
        })
    }

    pub fn get_transactions(identity: &Principal) -> Option<Vec<Transaction>> {
        let user_transactions =
            USER_TRANSACTIONS.with_borrow(|k| k.get(identity).and_then(|x| Some(x.clone())))?;

        Some(
            user_transactions
                .iter()
                .filter_map(|transaction_id| Self::get_transaction(identity, *transaction_id))
                .collect::<Vec<_>>(),
        )
    }

    pub fn delete_transaction(
        identity: &Principal,
        transaction_id: TransactionId,
    ) -> Option<Vec<StorableWrapper<Instruction>>> {
        USER_TRANSACTIONS.with_borrow_mut(|k| {
            let mut before = k.get(identity).unwrap_or_default();
            before.retain(|x| *x != transaction_id);
            k.insert(*identity, before);
        });

        let instructions_to_remove = TRANSACTIONS.with_borrow_mut(|k| k.remove(&transaction_id))?;

        Some(
            instructions_to_remove
                .iter()
                .filter_map(|i| INSTRUCTIONS.with_borrow_mut(|k| k.remove(i)))
                .collect::<Vec<_>>(),
        )
    }
}
