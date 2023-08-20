use std::error::Error;

use serde::{de::DeserializeOwned, Serialize};

pub trait SerializerApi<T>
where
    T: Serialize + DeserializeOwned,
{
    fn serialize(&mut self, obj: &T) -> Result<(), Box<dyn Error>>;
    fn deserialize(&mut self) -> Result<T, Box<dyn Error>>;
}
