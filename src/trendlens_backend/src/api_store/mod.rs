use crate::{
    exchange::Exchange,
    memory::{Memory, MemoryLocation, MEMORY_MANAGER},
    storable_wrapper::StorableWrapper,
};
use candid::{CandidType, Principal};
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap};

type ApiKey = String;
type ApiKeysStore = StableBTreeMap<Principal, StorableWrapper<HashMap<ApiKey, ApiData>>, Memory>;

thread_local! {
  static API_KEYS: RefCell<ApiKeysStore> = RefCell::new(
      StableBTreeMap::init(
          MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::UserKeys.memory_id())),
      )
  );
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ApiData {
    pub exchange: Exchange,
    pub api_key: String,
    pub passphrase: Option<String>,
}

pub struct ApiStore {}

impl ApiStore {
    pub fn register_key(identity: &Principal, data: ApiData) {
        API_KEYS.with_borrow_mut(|k| {
            if let Some(user_keys) = &mut k.get(&identity) {
                user_keys.insert(data.api_key.clone(), data);
            } else {
                let mut store = HashMap::new();
                store.insert(data.api_key.clone(), data);
                k.insert(identity.clone(), StorableWrapper(store));
            }
        });
    }

    pub fn remove_key(identity: &Principal, api_key: &String) {
        API_KEYS.with_borrow_mut(|k| {
            if let Some(user_keys) = &mut k.get(&identity) {
                user_keys.remove(api_key);
            }
        })
    }

    pub fn get_all_keys(identity: &Principal) -> Option<Vec<ApiData>> {
        API_KEYS.with_borrow(|k| Some(k.get(&identity)?.values().cloned().collect()))
    }

    pub fn get_by_api(identity: &Principal, api_key: &str) -> Option<ApiData> {
        API_KEYS.with_borrow(|k| k.get(&identity)?.get(api_key).cloned())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_register_key() {
    //     let principal = Principal::from_text("aaaaa-aa").unwrap();

    //     let data = ApiData {
    //         exchange: Exchange::Okx,
    //         api_key: "api_key".to_string(),
    //         passphrase: Some("passphrase".to_string()),
    //     };
    //     ApiStore::register_key(&principal, data.clone());
    //     let keys = API_KEYS.with_borrow(|k| k.get(&principal).unwrap());
    //     assert_eq!(keys.len(), 1);
    //     assert_eq!(keys[0], data);
    // }
}
