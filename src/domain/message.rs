use serenity::async_trait;

#[async_trait]
pub trait Rate {
    async fn get_rate(currency: &str, amount: f64) -> (f64, f64);
}

#[async_trait]
pub trait RateUsecase {
    async fn get_rate(currency: &str, amount: f64) -> (String, String);
}
