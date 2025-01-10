use std::{collections::HashMap, ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>, // make AppState clone to be Lightweight
}

#[derive(Debug, Clone)]
pub struct Exchange {
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) rate: f64,
}

#[derive(Debug)]
pub struct AppStateInner {
    // exchange rate
    pub exchange_rate: HashMap<String, Exchange>,
    pub symbols: HashMap<String, ()>,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new() -> Self {
        let mut symbols = HashMap::new();
        symbols.insert("USD".to_string(), ());
        symbols.insert("YUAN".to_string(), ());
        symbols.insert("EUR".to_string(), ());

        let mut exchange_rate = HashMap::new();
        exchange_rate.insert(
            "YUAN".to_string(),
            Exchange {
                from: "YUAN".to_string(),
                to: "USD".to_string(),
                rate: 0.15,
            },
        );
        exchange_rate.insert(
            "USD".to_string(),
            Exchange {
                from: "USD".to_string(),
                to: "YUAN".to_string(),
                rate: 6.7,
            },
        );

        Self {
            inner: Arc::new(AppStateInner {
                exchange_rate,
                symbols,
            }),
        }
    }
}
