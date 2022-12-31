use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;

use crate::domain::message::{FoodUsecase, RateUsecase};
use crate::usecase::message::MessageUsecase;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!") == false {
            return;
        }
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
                let message = MessageUsecase::echo(args[1].to_string());
                send_message(ctx, msg.channel_id, message).await;
            }
            "!rate" => {
                let curr: &str;
                let mut amount = 1.0;

                match args.len() {
                    1 => {
                        send_message(ctx, msg.channel_id, "是要哪個幣?".to_string()).await;
                        return;
                    }
                    2 => {
                        curr = args[1];
                    }
                    _ => {
                        curr = args[1];
                        amount = args[2].parse::<f64>().unwrap_or(1.0);
                    }
                }

                let (result, rate) = MessageUsecase::get_rate(curr, amount).await;
                let message = format!("{}/{}\nrate: {}", result, curr, rate);
                send_message(ctx, msg.channel_id, message).await;
            }
            "!food" => {
                let mut count = 1;
                if args.len() > 2 {
                    count = args[1].parse::<usize>().unwrap();
                }
                let message = MessageUsecase::get_food(count).await;
                send_message(ctx, msg.channel_id, message).await;
            }
            _ => {
                let message = format!("{}", "會不會用?");
                send_message(ctx, msg.channel_id, message).await;
            }
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
