use serenity::async_trait;

#[async_trait]
pub trait Rate {
    async fn get_rate(currency: &str, amount: f64) -> (f64, f64);
}

#[async_trait]
pub trait RateUsecase {
    async fn get_rate(args: Vec<&str>) -> String;
}

#[async_trait]
pub trait Food {
    async fn get_food(num: usize) -> Vec<String>;
}

#[async_trait]
pub trait FoodUsecase {
    async fn get_food(args: Vec<&str>) -> String;
}

#[async_trait]
pub trait LolUsecase {
    async fn ping_channel(args: Vec<&str>) -> String;
}

#[async_trait]
pub trait SystemUsecase {
    async fn get_version(args: Vec<&str>) -> String;
}
