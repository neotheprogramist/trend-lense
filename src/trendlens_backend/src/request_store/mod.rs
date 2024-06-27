use crate::{
    memory::{Memory, MemoryLocation, MEMORY_MANAGER},
    storable_wrapper::StorableWrapper,
};
use candid::Principal;
use ic_stable_structures::StableBTreeMap;
use request::Request;
use std::cell::RefCell;

pub mod request;

const MAX_USER_REQUESTS: usize = 10;
type AwaitingRequestsStore =
    StableBTreeMap<Principal, StorableWrapper<(u8, [Request; MAX_USER_REQUESTS])>, Memory>;

thread_local! {
pub static API_KEYS: RefCell<AwaitingRequestsStore> = RefCell::new(
    StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::ApiKeys.memory_id())),
    )
);
}

// consider making users and then implementing it as trait not using struct but directly
// use getters to extract useful information from contract state
pub struct RequestStore {}

impl RequestStore {
    pub fn add_request(identity: &Principal, api_key: String, data: Request) -> u8 {
        API_KEYS.with_borrow_mut(|k| {
            if let Some((index, user_keys)) = k.get(&identity).as_deref_mut() {
                let new_index = (*index as usize + 1) % MAX_USER_REQUESTS as usize;
                user_keys[new_index] = data;

                new_index as u8
            } else {
                k.insert(
                    identity.clone(),
                    StorableWrapper((0, [(); MAX_USER_REQUESTS].map(|_| Request::default()))),
                );

                0u8
            }
        })
    }

    pub fn get_request(identity: &Principal, index: usize) -> Option<Request> {
        API_KEYS.with_borrow(|k| k.get(&identity)?.1.get(index).cloned())
    }
}
