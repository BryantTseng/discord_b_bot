use serenity::async_trait;

#[async_trait]
pub trait Rate {
    async fn get_rate(currency: &str, amount: f64) -> (f64, f64);
}

#[async_trait]
pub trait RateUsecase {
    async fn get_rate(currency: &str, amount: f64) -> (String, String);
}

#[async_trait]
pub trait Food {
    async fn get_food(num: usize) -> Vec<String>;
}

#[async_trait]
pub trait FoodUsecase {
    async fn get_food(count: usize) -> String;
}
