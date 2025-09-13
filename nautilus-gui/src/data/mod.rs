use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

pub mod providers;
pub mod websocket;

use crate::models::{MarketData, OrderBook};

#[async_trait]
pub trait DataProvider: Send + Sync {
    async fn fetch_historical_data(
        &self,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        timeframe: &str,
    ) -> Result<Vec<MarketData>>;
    
    async fn fetch_latest_price(&self, symbol: &str) -> Result<Decimal>;
    
    async fn fetch_orderbook(&self, symbol: &str) -> Result<OrderBook>;
    
    async fn subscribe_market_data(&self, symbols: Vec<String>) -> Result<()>;
}

pub struct DataManager {
    provider: Box<dyn DataProvider>,
    cache: HashMap<String, Vec<MarketData>>,
}

impl DataManager {
    pub fn new(provider: Box<dyn DataProvider>) -> Self {
        Self {
            provider,
            cache: HashMap::new(),
        }
    }
    
    pub async fn get_historical_data(
        &mut self,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        timeframe: &str,
    ) -> Result<Vec<MarketData>> {
        // Check cache first
        let cache_key = format!("{}-{}-{}-{}", symbol, start, end, timeframe);
        
        if let Some(cached_data) = self.cache.get(&cache_key) {
            return Ok(cached_data.clone());
        }
        
        // Fetch from provider
        let data = self.provider.fetch_historical_data(symbol, start, end, timeframe).await?;
        
        // Cache the data
        self.cache.insert(cache_key, data.clone());
        
        Ok(data)
    }
    
    pub async fn get_latest_price(&self, symbol: &str) -> Result<Decimal> {
        self.provider.fetch_latest_price(symbol).await
    }
    
    pub async fn subscribe_to_symbols(&self, symbols: Vec<String>) -> Result<()> {
        self.provider.subscribe_market_data(symbols).await
    }
}