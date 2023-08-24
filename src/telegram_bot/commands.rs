use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Add new search for the scraping.", parse_with = "split")]
    Add { name: String, query: String },
    #[command(description = "List all searches.")]
    List,
}
