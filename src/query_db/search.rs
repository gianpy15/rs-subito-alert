#[derive(Clone, Debug, PartialEq)]
pub struct Search {
    name: String,
    query: String,
}

impl Search {
    pub fn new(name: String, query: String) -> Search {
        Search { name, query }
    }
}
