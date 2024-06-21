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
