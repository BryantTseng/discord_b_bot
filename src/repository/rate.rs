use serde::Deserialize;
use serenity::async_trait;

use crate::domain::message::Rate;

pub struct RateRepository;

#[async_trait]
impl Rate for RateRepository {
    async fn get_rate(currency: &str, amount: f64) -> (f64, f64) {
        let url = format!(
            "https://api.exchangerate.host/convert?amount={}&from={}&to=TWD",
            amount, currency
        );

        let resp = reqwest::get(url)
            .await
            .unwrap()
            .json::<QueryCurrencyResult>()
            .await
            .unwrap();
        (resp.result, resp.info.rate)
    }
}
#[allow(dead_code)]
#[derive(Deserialize)]
struct QueryCurrencyResult {
    motd: Motd,
    success: bool,
    query: Query,
    info: Info,
    historical: bool,
    date: String,
    result: f64,
}
#[allow(dead_code)]
#[allow(dead_code)]
#[derive(Deserialize)]
struct Motd {
    msg: String,
    url: String,
}
#[allow(dead_code)]
#[derive(Deserialize)]
struct Query {
    from: String,
    to: String,
    amount: f64,
}
#[derive(Deserialize)]
struct Info {
    rate: f64,
}
