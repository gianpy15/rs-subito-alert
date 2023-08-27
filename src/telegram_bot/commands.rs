use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Add new search for the scraping.")]
    Add,
    #[command(description = "Delete a search.")]
    Delete,
    #[command(description = "List all searches.")]
    List,
    #[command(description = "Cancel current bot interaction.")]
    Cancel,
}
