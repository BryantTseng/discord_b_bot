use std::env;

use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub discord_token: String,
    //pub fugle_token: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();
        let discord_token =
            env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
        //let fugle_token =
        //    env::var("FUGLE_TOKEN").expect("Expected a FUGLE_TOKEN in the environment");
        Self {
            discord_token,
            //   fugle_token,
        }
    }
}
