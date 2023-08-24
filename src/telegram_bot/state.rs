#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveSearchName {
        name: String,
    },
    ReceiveSearchQuery {
        query: String,
    },
}
