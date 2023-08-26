#[derive(Clone, Default)]
pub enum State {
    #[default]
    Idle,
    ReceiveSearchName,
    ReceiveSearchQuery {
        search_name: String,
    },
}
