#[derive(Clone, Default)]
pub enum State {
    #[default]
    Idle,
    ReceiveSearchName,
    ReceiveSearchQuery {
        search_name: String,
    },
    ReceiveSearchPrice {
        search_name: String,
        search_query: String,
    },
    SelectDelete,
    Delete,
}
