use ic_stable_structures::{
    memory_manager::{MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};
use std::cell::RefCell;

mod constants;
mod location;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub use location::*;
