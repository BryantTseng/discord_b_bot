use serenity::async_trait;

use crate::domain::message::{Food, FoodUsecase, LolUsecase, Rate, RateUsecase};
use crate::repository::food::FoodRepository;
use crate::repository::rate::RateRepository;
pub struct MessageUsecase;

impl MessageUsecase {
    pub fn echo(args: Vec<&str>) -> String {
        args[0].to_string()
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

#[async_trait]
impl FoodUsecase for MessageUsecase {
    async fn get_food(args: Vec<&str>) -> String {
        let mut amount: usize = 1;
        if args.len() >= 1 {
            amount = args[0].parse::<usize>().unwrap();
        }
        let foods = FoodRepository::get_food(amount).await;
        foods.join(",")
    }
}

#[async_trait]
impl LolUsecase for MessageUsecase {
    async fn ping_channel(args: Vec<&str>) -> String {
        let mut people_need = 4;
        if let Some(e) = args.get(0) {
            people_need = e.parse::<u32>().unwrap_or(4);
        }
        //let people_need = args[0].parse::<u32>().unwrap_or(4);
        format!("@everyone 打Lol -{}", people_need)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    #[tokio::test]
    async fn get_jpy_rate_string() {
        let r = MessageUsecase::get_rate(vec!["JPY", "3000"]).await;

        let re = Regex::new(r"[0-9.]+/JPY\nrate: [0-9.]+").unwrap();
        assert!(re.is_match(&r));
    }
    #[tokio::test]
    async fn get_food_string() {
        let r = MessageUsecase::get_food(vec!["1"]).await;
        assert!(r.is_empty() == false)
    }
    #[tokio::test]
    async fn get_food_string_without_number() {
        let r = MessageUsecase::get_food(vec![]).await;
        assert!(r.is_empty() == false)
    }
}
