#[derive(Clone, Debug, PartialEq)]
pub struct Search {
    pub name: String,
    pub query: String,
}

impl Search {
    pub fn new(name: String, query: String) -> Search {
        Search { name, query }
    }
}
