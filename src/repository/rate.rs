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

        //let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        let resp = reqwest::get(url)
            .await
            .unwrap()
            .json::<QueryCurrencyResult>()
            .await
            .unwrap();
        //println!("{:#?}", resp);
        (resp.result, resp.info.rate)
    }
}
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
#[derive(Deserialize)]
struct Motd {
    msg: String,
    url: String,
}
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
