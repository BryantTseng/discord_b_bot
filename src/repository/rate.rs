use serde::Deserialize;

pub struct RateRepository;

impl RateRepository {
    pub async fn get_rate(currency: &str, amount: f64) -> (f64, f64) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_jpy_rate() {
        let (result, rate) = RateRepository::get_rate("JPY", 1.0).await;

        assert!(result.is_normal());
        assert!(rate.is_normal());
    }
}
