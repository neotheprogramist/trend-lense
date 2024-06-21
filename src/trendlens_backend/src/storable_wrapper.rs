use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Default)]
pub struct StorableWrapper<T>(pub T)
where
    T: for<'de> Deserialize<'de>;

impl<T> std::ops::Deref for StorableWrapper<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for StorableWrapper<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Storable for StorableWrapper<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        let buf = bincode::serialize(&self.0).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        StorableWrapper(bincode::deserialize(bytes.as_ref()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Test {
        a: u32,
        b: String,
        c: u64
    }

    #[test]
    fn test_serialize() {
        let test_struct = Test {
            a: 42,
            b: "owner".to_string(),
            c: 1234567890
        };
        let storable_wrapper = StorableWrapper(test_struct.clone());
        let bytes = storable_wrapper.to_bytes();
        let storable_wrapper2: StorableWrapper<Test> = StorableWrapper::from_bytes(bytes);
        assert_eq!(*storable_wrapper, *storable_wrapper2);
    }
}
