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

#[derive(Deserialize, Serialize, Clone)]
pub struct UserRequests {
    pub next_index: u8,
    pub requests: [ExchangeRequestInfo; MAX_USER_REQUESTS],
}

type AwaitingRequestsStore = StableBTreeMap<Principal, StorableWrapper<UserRequests>, Memory>;

thread_local! {
    static REQUESTS: RefCell<AwaitingRequestsStore> = RefCell::new(
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
        REQUESTS.with_borrow_mut(|k| {
            if let Some(user_requests) = &mut k.get(&identity) {
                let index = user_requests.next_index as usize;
                ic_cdk::println!("some: before {}", index);

                user_requests.requests[index] = request;
                user_requests.next_index = ((index + 1) % MAX_USER_REQUESTS) as u8;

                ic_cdk::println!("some: next {}", user_requests.next_index);

                index as u8
            } else {
                let mut array = [(); MAX_USER_REQUESTS].map(|_| ExchangeRequestInfo {
                    api_key: String::new(),
                    exchange: Exchange::Coinbase,
                    request: Request::Empty,
                });

                array[0] = request;

                k.insert(
                    identity.clone(),
                    StorableWrapper(UserRequests {
                        next_index: 1,
                        requests: array,
                    }),
                );

                ic_cdk::println!("none: next {}", 1);

                0
            }
        })
    }

    pub fn next_index(identity: &Principal) -> u8 {
        REQUESTS.with_borrow(|k| {
            if let Some(i) = k.get(&identity).and_then(|x| Some(x.0)) {
                i.next_index
            } else {
                0
            }
        })
    }

    pub fn get_request(identity: &Principal, index: u8) -> Option<ExchangeRequestInfo> {
        REQUESTS.with_borrow(|k| k.get(&identity)?.requests.get(index as usize).cloned())
    }

    pub fn delete_request(identity: &Principal, index: u8) {
        REQUESTS.with_borrow_mut(|k| {
            if let Some(user_requests) = &mut k.get(&identity) {
                user_requests.requests[index as usize] = ExchangeRequestInfo {
                    api_key: String::new(),
                    exchange: Exchange::Coinbase,
                    request: Request::Empty,
                };
            }
        })
    }
}
