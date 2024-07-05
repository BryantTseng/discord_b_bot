use serenity::{
    all::{Context, EventHandler, GatewayIntents, Message, Ready},
    async_trait, Client,
};
use url::Url;

use crate::{usecase::message::MessageUsecase, utils::config::Config};

pub struct Discord {
    config: Config,
    client: Client,
    handler: DiscordHandler,
}
pub struct DiscordHandler {}

#[async_trait]
impl EventHandler for DiscordHandler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with('!') {
            let res = MessageUsecase::execute_command(msg.content);
        } else if let Ok(url) = Url::parse(&msg.content) {
            let res = MessageUsecase::execute_url(url);
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

impl Discord {
    pub async fn new(config: Config) -> Self {
        // Set gateway intents, which decides what events the bot will be notified about
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        // Create a new instance of the Client, logging in as a bot. This will
        // automatically prepend your bot token with "Bot ", which is a requirement
        // by Discord for bot users.
        let client = Client::builder(&config.discord_token, intents)
            .event_handler(DiscordHandler {})
            .await
            .expect("Err creating client");

        Self {
            config,
            client,
            handler: DiscordHandler {},
        }
    }

    pub async fn start(&mut self) {
        // Finally, start a single shard, and start listening to events.
        //
        // Shards will automatically attempt to reconnect, and will perform
        // exponential backoff until it reconnects.
        if let Err(why) = self.client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}
