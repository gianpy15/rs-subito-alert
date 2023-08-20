use std::error::Error;

use serde::{Serialize, de::DeserializeOwned};

pub trait SerializerApi<T>
where
T: Serialize + DeserializeOwned {
    fn serialize(&mut self, obj: &T) -> Result<(), Box<dyn Error>>;
    fn deserialize(&mut self) -> Result<T, Box<dyn Error>>;
}
