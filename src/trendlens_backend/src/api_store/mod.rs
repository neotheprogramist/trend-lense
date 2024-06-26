use crate::{
    exchange::Exchange,
    memory::{Memory, MemoryLocation, MEMORY_MANAGER},
    storable_wrapper::StorableWrapper,
};
use candid::{CandidType, Principal};
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap};

type ExchangeApiMap = HashMap<u8, Vec<ApiData>>;
type ApiKeysStore = StableBTreeMap<Principal, StorableWrapper<ExchangeApiMap>, Memory>;

thread_local! {
  pub static API_KEYS: RefCell<ApiKeysStore> = RefCell::new(
      StableBTreeMap::init(
          MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::ApiKeys.memory_id())),
      )
  );
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ApiData {
    api_key: String,
    passphrase: Option<String>,
}

pub struct ApiStore {}

impl ApiStore {
    pub fn register_key(identity: &Principal, exchange: Exchange, data: ApiData) {
        API_KEYS.with_borrow_mut(|k| {
            if let Some(user_keys) = &mut k.get(&identity) {
                if let Some(exchange_keys) = user_keys.get_mut(&exchange.into()) {
                    exchange_keys.push(data);
                } else {
                    user_keys.insert(exchange.into(), vec![data]);
                }
            } else {
                let mut exchange_keys = HashMap::new();
                exchange_keys.insert(exchange.into(), vec![data]);
                k.insert(identity.clone(), StorableWrapper(exchange_keys));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_key() {
        let principal = Principal::from_text("aaaaa-aa").unwrap();

        let data = ApiData {
            api_key: "api_key".to_string(),
            passphrase: Some("passphrase".to_string()),
        };
        ApiStore::register_key(&principal, Exchange::Okx, data.clone());
        let keys = API_KEYS.with_borrow(|k| k.get(&principal).unwrap());
        assert_eq!(keys.len(), 1);
        assert_eq!(keys.get(&Exchange::Okx.into()).unwrap()[0], data);
    }
}
