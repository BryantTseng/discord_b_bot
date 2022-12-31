use serenity::async_trait;

use crate::domain::message::{Food, FoodUsecase, Rate, RateUsecase};
use crate::repository::food::FoodRepository;
use crate::repository::rate::RateRepository;
pub struct MessageUsecase;

impl MessageUsecase {
    pub fn echo(s: String) -> String {
        s
    }
}

#[async_trait]
impl RateUsecase for MessageUsecase {
    async fn get_rate(s: &str, amount: f64) -> (String, String) {
        let (result, rate) = RateRepository::get_rate(&s, amount).await;
        (format!("{:.2}", result), format!("{:.5}", rate))
    }
}

#[async_trait]
impl FoodUsecase for MessageUsecase {
    async fn get_food(count: usize) -> String {
        let foods = FoodRepository::get_food(count).await;
        foods.join(",")
    }
}
