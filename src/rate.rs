use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use crate::errors::AppError;

const RATE_URL: &'static str =
    "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies/";

pub struct RateGetter {
    pub from: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromExchange {
    pub from: String,
    pub currency_rate: HashMap<String, f64>,
}

impl FromExchange {
    pub fn new(from: String, currency_rate: HashMap<String, f64>) -> Self {
        Self {
            from,
            currency_rate,
        }
    }

    pub fn get_rate(&self, to: &str) -> Option<&f64> {
        self.currency_rate.get(to)
    }
}

impl RateGetter {
    pub fn new(from: String) -> Self {
        Self { from }
    }

    pub async fn get_exc_rate(&self) -> Result<FromExchange, AppError> {
        let suffix = format!("{}.json", self.from.to_lowercase());
        let request_url = format!("{}{}", RATE_URL, suffix);
        info!("request_url: {}", request_url);

        match reqwest::get(&request_url).await {
            Ok(resp) => {
                if resp.status().is_success() {
                    let data = resp.text().await.unwrap();

                    let obj: Value = serde_json::from_str(&data).unwrap();
                    let cur_rates: &Value = &obj[&self.from.to_lowercase()];

                    let exchange = FromExchange {
                        from: self.from.clone(),
                        currency_rate: cur_rates.as_object().unwrap().iter().fold(
                            HashMap::new(),
                            |mut acc, (k, v)| {
                                acc.insert(k.clone(), v.as_f64().unwrap());
                                acc
                            },
                        ),
                    };

                    Ok(exchange)
                } else {
                    Err(AppError::GetRateError(resp.text().await.unwrap()))
                }
            }
            Err(e) => Err(AppError::ConvertError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_rate() {
        let rate_getter = RateGetter::new("USD".to_string());
        let exc = rate_getter.get_exc_rate().await.unwrap();
        assert_eq!(*exc.currency_rate.get("cny").unwrap() > 7.0, true);
    }
}
