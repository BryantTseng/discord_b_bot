use regex::Regex;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;

use crate::domain::message::RateUsecase;
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
        let re = Regex::new(r"!() .*$").unwrap();
        let command: String = match re.find(&msg.content) {
            Some(v) => v.as_str().to_string(),
            None => "unknown".to_string(),
        };

        match command.as_str() {
            "ping" => {
                send_message(ctx, msg.channel_id, "pong".to_string()).await;
            }
            "echo" => {
                let message = MessageUsecase::echo(msg.content[6..].to_string());
                send_message(ctx, msg.channel_id, message).await;
            }
            "rate" => {
                let r = MessageUsecase::echo(msg.content[6..].to_string());
                let s = r.split_whitespace();
                let mut curr = String::new();
                let mut amount: f64 = 1.0;

                let mut count = 0;
                for each in s {
                    if count == 0 {
                        curr = each.to_string();
                    } else if count == 1 {
                        amount = each.parse::<f64>().unwrap();
                    } else {
                        break;
                    }
                    count += 1;
                }
                let (result, rate) = MessageUsecase::get_rate(curr, amount).await;
                let message = format!("{},{}", result, rate);
                send_message(ctx, msg.channel_id, message).await;
            }
            "unknown" => {
                let message = format!("{}", "會不會用?");
                send_message(ctx, msg.channel_id, message).await;
            }
            _ => {}
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
