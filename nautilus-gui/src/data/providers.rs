use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::data::DataProvider;
use crate::models::{MarketData, OrderBook, OrderBookLevel};

// Alpha Vantage Provider
pub struct AlphaVantageProvider {
    api_key: String,
    client: Client,
}

impl AlphaVantageProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct AlphaVantageResponse {
    #[serde(rename = "Time Series (1min)")]
    time_series: Option<std::collections::HashMap<String, AlphaVantageCandle>>,
}

#[derive(Debug, Deserialize)]
struct AlphaVantageCandle {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

#[async_trait]
impl DataProvider for AlphaVantageProvider {
    async fn fetch_historical_data(
        &self,
        symbol: &str,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
        timeframe: &str,
    ) -> Result<Vec<MarketData>> {
        let interval = match timeframe {
            "1m" => "1min",
            "5m" => "5min",
            "15m" => "15min",
            "30m" => "30min",
            "1h" => "60min",
            _ => "1min",
        };
        
        let url = format!(
            "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval={}&apikey={}",
            symbol, interval, self.api_key
        );
        
        let response = self.client.get(&url).send().await?;
        let data: AlphaVantageResponse = response.json().await?;
        
        let mut market_data = Vec::new();
        
        if let Some(time_series) = data.time_series {
            for (timestamp_str, candle) in time_series {
                let timestamp = DateTime::parse_from_str(&format!("{} +0000", timestamp_str), "%Y-%m-%d %H:%M:%S %z")?
                    .with_timezone(&Utc);
                
                market_data.push(MarketData {
                    symbol: symbol.to_string(),
                    timestamp,
                    open: Decimal::from_str(&candle.open)?,
                    high: Decimal::from_str(&candle.high)?,
                    low: Decimal::from_str(&candle.low)?,
                    close: Decimal::from_str(&candle.close)?,
                    volume: Decimal::from_str(&candle.volume)?,
                    bid: None,
                    ask: None,
                    bid_size: None,
                    ask_size: None,
                });
            }
        }
        
        market_data.sort_by_key(|d| d.timestamp);
        Ok(market_data)
    }
    
    async fn fetch_latest_price(&self, symbol: &str) -> Result<Decimal> {
        let url = format!(
            "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            symbol, self.api_key
        );
        
        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;
        
        if let Some(quote) = data.get("Global Quote") {
            if let Some(price_str) = quote.get("05. price").and_then(|p| p.as_str()) {
                return Ok(Decimal::from_str(price_str)?);
            }
        }
        
        Err(anyhow::anyhow!("Failed to fetch latest price"))
    }
    
    async fn fetch_orderbook(&self, _symbol: &str) -> Result<OrderBook> {
        // Alpha Vantage doesn't provide orderbook data in free tier
        Err(anyhow::anyhow!("Orderbook not available for Alpha Vantage"))
    }
    
    async fn subscribe_market_data(&self, _symbols: Vec<String>) -> Result<()> {
        // Alpha Vantage doesn't support WebSocket streaming in free tier
        Ok(())
    }
}

// Mock Provider for testing
pub struct MockProvider;

impl MockProvider {
    pub fn new() -> Self {
        Self
    }
    
    fn generate_mock_data(&self, symbol: &str, count: usize) -> Vec<MarketData> {
        let mut data = Vec::new();
        let mut base_price = Decimal::from(50000); // Starting price for BTC
        let now = Utc::now();
        
        for i in 0..count {
            let timestamp = now - chrono::Duration::minutes((count - i) as i64);
            let random_change = Decimal::from(rand::random::<f64>() * 1000.0 - 500.0);
            base_price += random_change;
            
            let high = base_price + Decimal::from(rand::random::<f64>() * 100.0);
            let low = base_price - Decimal::from(rand::random::<f64>() * 100.0);
            let open = base_price + Decimal::from(rand::random::<f64>() * 50.0 - 25.0);
            let close = base_price + Decimal::from(rand::random::<f64>() * 50.0 - 25.0);
            let volume = Decimal::from(rand::random::<f64>() * 1000.0);
            
            data.push(MarketData {
                symbol: symbol.to_string(),
                timestamp,
                open,
                high,
                low,
                close,
                volume,
                bid: Some(close - Decimal::from(1)),
                ask: Some(close + Decimal::from(1)),
                bid_size: Some(Decimal::from(rand::random::<f64>() * 10.0)),
                ask_size: Some(Decimal::from(rand::random::<f64>() * 10.0)),
            });
        }
        
        data
    }
}

#[async_trait]
impl DataProvider for MockProvider {
    async fn fetch_historical_data(
        &self,
        symbol: &str,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
        _timeframe: &str,
    ) -> Result<Vec<MarketData>> {
        // Generate mock data
        Ok(self.generate_mock_data(symbol, 100))
    }
    
    async fn fetch_latest_price(&self, _symbol: &str) -> Result<Decimal> {
        Ok(Decimal::from(50000 + rand::random::<i32>() % 1000))
    }
    
    async fn fetch_orderbook(&self, symbol: &str) -> Result<OrderBook> {
        let mid_price = Decimal::from(50000);
        let mut bids = Vec::new();
        let mut asks = Vec::new();
        
        for i in 1..=10 {
            bids.push(OrderBookLevel {
                price: mid_price - Decimal::from(i * 10),
                size: Decimal::from(rand::random::<f64>() * 10.0),
                orders: Some(rand::random::<u32>() % 10 + 1),
            });
            
            asks.push(OrderBookLevel {
                price: mid_price + Decimal::from(i * 10),
                size: Decimal::from(rand::random::<f64>() * 10.0),
                orders: Some(rand::random::<u32>() % 10 + 1),
            });
        }
        
        Ok(OrderBook {
            symbol: symbol.to_string(),
            timestamp: Utc::now(),
            bids,
            asks,
        })
    }
    
    async fn subscribe_market_data(&self, _symbols: Vec<String>) -> Result<()> {
        Ok(())
    }
}