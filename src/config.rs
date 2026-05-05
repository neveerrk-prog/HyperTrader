use anyhow::Result;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub api_secret: String,
    pub base_url: String,
    pub ws_base: String,
    pub db_path: String,
    pub trading_pairs: Vec<String>,
    pub bid_spread: f64,
    pub ask_spread: f64,
    pub order_quantity: f64,
    pub max_position: f64,
    pub refresh_interval_ms: u64,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        let api_key = env::var("BINANCE_API_KEY").unwrap_or_default();
        let api_secret = env::var("BINANCE_API_SECRET").unwrap_or_default();
        let base_url = env::var("BINANCE_BASE_URL").unwrap_or_else(|_| "https://testnet.binance.vision".into());
        let ws_base = env::var("BINANCE_WS_BASE").unwrap_or_else(|_| "wss://testnet.binance.vision/ws".into());
        let db_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "./data/hypertrader.db".into());
        let trading_pairs = env::var("TRADING_PAIRS").unwrap_or_else(|_| "BTCUSDT,ETHUSDT".into())
            .split(',')
            .map(|s| s.trim().to_uppercase())
            .filter(|s| !s.is_empty())
            .collect();
        let bid_spread = env::var("BID_SPREAD").unwrap_or_else(|_| "0.1".into()).parse().unwrap_or(0.1);
        let ask_spread = env::var("ASK_SPREAD").unwrap_or_else(|_| "0.1".into()).parse().unwrap_or(0.1);
        let order_quantity = env::var("ORDER_QUANTITY").unwrap_or_else(|_| "0.001".into()).parse().unwrap_or(0.001);
        let max_position = env::var("MAX_POSITION").unwrap_or_else(|_| "0.1".into()).parse().unwrap_or(0.1);
        let refresh_interval_ms = env::var("REFRESH_INTERVAL_MS").unwrap_or_else(|_| "1000".into()).parse().unwrap_or(1000);
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into());

        Ok(Self {
            api_key,
            api_secret,
            base_url,
            ws_base,
            db_path,
            trading_pairs,
            bid_spread,
            ask_spread,
            order_quantity,
            max_position,
            refresh_interval_ms,
            log_level,
        })
    }
}
