use std::rc::Rc;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Search {
    pub name: Rc<String>,
    pub query: Rc<String>,
}

impl Search {
    pub fn new(name: String, query: String) -> Search {
        Search {
            name: Rc::new(name),
            query: Rc::new(query),
        }
    }
}
