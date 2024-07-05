use crate::{
    exchange::Exchange,
    memory::{Memory, MemoryLocation, MEMORY_MANAGER},
    storable_wrapper::StorableWrapper,
};
use candid::{CandidType, Principal};
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

type ApiId = u32;
type ApiKey = String;
type UserApiKeysTable = StableBTreeMap<(Principal, ApiId), StorableWrapper<ApiData>, Memory>;
type ApiKeysTable = StableBTreeMap<ApiKey, ApiId, Memory>;

thread_local! {
  static API_KEYS_INDEX: RefCell<ApiKeysTable> =  RefCell::new(
    StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::ApiKeys.memory_id())),
    )
);

  static USER_API_KEYS: RefCell<UserApiKeysTable> = RefCell::new(
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
    pub fn register_key(identity: &Principal, data: ApiData) -> Option<ApiId> {
        // first insert the key into table with keys mapped to ids
        let api_key = data.api_key.clone();

        let id = USER_API_KEYS.with_borrow_mut(|a| {
            let insert_index = a
                .last_key_value()
                .and_then(|((_p, id), _v)| Some(id))
                .unwrap_or(0)
                .checked_add(1)?;

            a.insert((*identity, insert_index), StorableWrapper(data));

            Some(insert_index)
        })?;

        API_KEYS_INDEX.with_borrow_mut(|k| {
            k.insert(api_key, id);
        });

        Some(id)
    }

    fn get_id(api_key: &String) -> Option<ApiId> {
        API_KEYS_INDEX.with_borrow(|k| k.get(api_key))
    }

    pub fn remove_key(identity: &Principal, api_key: &String) -> Option<ApiData> {
        let id = Self::get_id(api_key)?;

        USER_API_KEYS.with_borrow_mut(|k| k.remove(&(*identity, id)).as_deref().cloned())
    }

    pub fn get_by_api(identity: &Principal, api_key: &String) -> Option<ApiData> {
        let id = Self::get_id(api_key)?;

        USER_API_KEYS.with_borrow(|k| k.get(&(*identity, id)).as_deref().cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_key() {
        let principal = Principal::from_text("aaaaa-aa").unwrap();

        let data = ApiData {
            exchange: Exchange::Okx,
            api_key: "api_key".to_string(),
            passphrase: Some("passphrase".to_string()),
        };

        ApiStore::register_key(&principal, data.clone());

        assert_eq!(
            ApiStore::get_by_api(&principal, &data.api_key).unwrap(),
            data
        );

        assert!(API_KEYS_INDEX.with_borrow(|k| k.get(&data.api_key)).is_some());
        assert!(USER_API_KEYS.with_borrow(|k| k.get(&(principal, 1))).is_some());
    }
}
