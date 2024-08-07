mod cli;
mod repository;
mod transport;
mod utils;

use transport::discord::Discord;
use utils::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load();
    let mut discord = Discord::new(config.clone()).await;
    discord.start().await;
}
