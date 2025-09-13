use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use self::market_data::*;
pub use self::order::*;
pub use self::position::*;
pub use self::strategy::*;

mod market_data;
mod order;
mod position;
mod strategy;

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_view: crate::app::ViewType,
    pub selected_symbol: Option<String>,
    pub selected_timeframe: String,
    pub market_data: Vec<MarketData>,
    pub positions: Vec<Position>,
    pub orders: Vec<Order>,
    pub strategies: Vec<Strategy>,
    pub account_balance: Decimal,
    pub is_connected: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_view: crate::app::ViewType::Dashboard,
            selected_symbol: None,
            selected_timeframe: "1m".to_string(),
            market_data: Vec::new(),
            positions: Vec::new(),
            orders: Vec::new(),
            strategies: Vec::new(),
            account_balance: Decimal::from(100000),
            is_connected: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub data_provider: DataProvider,
    pub theme: String,
    pub auto_save: bool,
    pub python_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataProvider {
    AlphaVantage { api_key: String },
    YahooFinance,
    Binance { api_key: String, secret: String },
    Mock,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "postgresql://localhost/nautilus_trader".to_string(),
            data_provider: DataProvider::Mock,
            theme: "dark".to_string(),
            auto_save: true,
            python_path: "python3".to_string(),
        }
    }
}