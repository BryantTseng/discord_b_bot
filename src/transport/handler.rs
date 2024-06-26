use crate::domain::message::{FoodUsecase, LolUsecase, RateUsecase, SystemUsecase};
use crate::usecase::message::MessageUsecase;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use url::Url;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with('!') {
            command_handler(ctx, &msg).await;
        } else if let Ok(url) = Url::parse(&msg.content) {
            url_handler(ctx, &msg, url).await;
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
async fn send_message(ctx: Context, channel_id: ChannelId, msg: String) {
    let _ = match channel_id.say(&ctx.http, msg).await {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    };
}

/// This handler would handle message start with '!'
async fn command_handler(ctx: Context, msg: &Message) {
    let command_split = msg.content.split_whitespace();
    let mut args: Vec<&str> = Vec::new();
    for each in command_split {
        args.push(each);
    }
    match args[0] {
        "!ping" => {
            send_message(ctx, msg.channel_id, "pong".to_string()).await;
        }
        "!echo" => {
            let message = MessageUsecase::echo(args[1..args.len()].to_vec());
            send_message(ctx, msg.channel_id, message).await;
        }
        "!rate" => {
            let message = MessageUsecase::get_rate(args[1..args.len()].to_vec()).await;
            send_message(ctx, msg.channel_id, message).await;
        }
        "!food" | "!吃啥" => {
            let message = MessageUsecase::get_food(args[1..args.len()].to_vec()).await;
            send_message(ctx, msg.channel_id, message).await;
        }
        "!version"| "!ver" => {
            let message = MessageUsecase::get_version(args[1..args.len()].to_vec()).await;
            send_message(ctx, msg.channel_id, message).await;
        }
        "!lol" => {
            let message = MessageUsecase::ping_channel(args[1..args.len()].to_vec()).await;
            send_message(ctx, msg.channel_id, message).await;
        }
        _ => {
            let message = format!("{}", "會不會用?");
            send_message(ctx, msg.channel_id, message).await;
        }
    }
}

/// This handler would handle message which is a valid URL
async fn url_handler(ctx: Context, msg: &Message, url: Url) {
    if let Some(host) = url.host() {
        match host.to_string().as_str() {
            "twitter.com" | "x.com" => {
                let message;
                let mut url = url.clone();
                if let Err(e) = url.set_host(Some("vxtwitter.com")) {
                    message = format!("fuck, {}", e);
                } else {
                    message = url.to_string();
                }

                send_message(ctx, msg.channel_id, message).await;
            }
            _ => {}
        }
    }
}
