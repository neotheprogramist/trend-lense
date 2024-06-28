use crate::{
    exchange::Exchange,
    memory::{Memory, MemoryLocation, MEMORY_MANAGER},
    storable_wrapper::StorableWrapper,
};
use candid::{CandidType, Principal};
use ic_stable_structures::StableBTreeMap;
use request::Request;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

pub mod request;

const MAX_USER_REQUESTS: usize = 10;

#[derive(Deserialize, Serialize, Clone, CandidType)]
pub struct ExchangeRequestInfo {
    pub exchange: Exchange,
    pub api_key: String,
    pub request: Request,
}

type AwaitingRequestsStore = StableBTreeMap<
    Principal,
    StorableWrapper<(u8, [ExchangeRequestInfo; MAX_USER_REQUESTS])>,
    Memory,
>;

thread_local! {
pub static API_KEYS: RefCell<AwaitingRequestsStore> = RefCell::new(
    StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::UserRequest.memory_id())),
    )
);
}

// consider making users and then implementing it as trait not using struct but directly
// use getters to extract useful information from contract state
pub struct RequestStore {}

impl RequestStore {
    pub fn add_request(identity: &Principal, request: ExchangeRequestInfo) -> u8 {
        API_KEYS.with_borrow_mut(|k| {
            let mut insert_index = 0;
            if let Some((index, user_keys)) = k.get(&identity).as_deref_mut() {
                insert_index = (*index as usize + 1) % MAX_USER_REQUESTS as usize;

                user_keys[insert_index] = request;
            } else {
                let mut array = [(); MAX_USER_REQUESTS].map(|_| ExchangeRequestInfo {
                    api_key: String::new(),
                    exchange: Exchange::Coinbase,
                    request: Request::Empty,
                });

                array[insert_index] = request;

                k.insert(identity.clone(), StorableWrapper((0, array)));
            };

            insert_index as u8
        })
    }

    pub fn get_request(identity: &Principal, index: u8) -> Option<ExchangeRequestInfo> {
        API_KEYS.with_borrow(|k| k.get(&identity)?.1.get(index as usize).cloned())
    }
}
