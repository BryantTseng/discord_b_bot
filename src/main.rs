mod domain;
mod repository;
mod transport;
mod usecase;
mod utils;

use transport::discord::Discord;
use utils::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::Load();
    let mut discord = Discord::new(config.clone()).await;
    discord.start().await;
}
