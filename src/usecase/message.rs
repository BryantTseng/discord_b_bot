use serenity::async_trait;
use url::Url;

use crate::domain::message::{Food, Rate, RateUsecase};
use crate::repository::food::FoodRepository;
use crate::repository::rate::RateRepository;
pub struct MessageUsecase;
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

impl MessageUsecase {
    pub fn echo(args: Vec<&str>) -> String {
        args[0].to_string()
    }
    pub async fn execute_command(msg: String) -> String {
        let command_split = msg.split_whitespace();
        let args: Vec<&str> = command_split.into_iter().collect();

        let res = match args[0] {
            "!ping" => "pong".to_string(),
            "!echo" => {
                if args.len() >= 2 {
                    args[1].to_string()
                } else {
                    "{EMPTY}".to_string()
                }
            }
            "!rate" => {
                let curr: &str;
                let mut amount = 1.0;

                match args.len() {
                    0 => return "是要什麼幣?".to_string(),
                    1 => {
                        curr = args[0];
                    }
                    _ => {
                        curr = args[0];
                        amount = args[1].parse::<f64>().unwrap_or(1.0);
                    }
                };
                let (result, rate) = RateRepository::get_rate(curr, amount).await;
                format!("{:.2}/{}\nrate: {:.5}", result, curr, rate)
            }
            "!food" | "!吃啥" => {
                let mut amount: usize = 1;
                if args.len() >= 1 {
                    amount = args[1].parse::<usize>().unwrap_or(1);
                }
                let foods = FoodRepository::get_food(amount).await;
                foods.join(",")
            }
            "!version" | "!ver" => CARGO_PKG_VERSION.to_string(),
            _ => {
                let message = format!("{}", "會不會用?");
                message
            }
        };
        return res;
    }
    pub fn execute_url(url: Url) -> String {
        todo!()
    }
}

#[async_trait]
impl RateUsecase for MessageUsecase {
    async fn get_rate(args: Vec<&str>) -> String {
        let curr: &str;
        let mut amount = 1.0;

        match args.len() {
            0 => {
                return "是要什麼幣?".to_string();
            }
            1 => {
                curr = args[0];
            }
            _ => {
                curr = args[0];
                amount = args[1].parse::<f64>().unwrap_or(1.0);
            }
        }
        let (result, rate) = RateRepository::get_rate(curr, amount).await;
        format!("{:.2}/{}\nrate: {:.5}", result, curr, rate)
    }
}
