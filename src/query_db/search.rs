use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
