use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub code: String,
    pub language: StrategyLanguage,
    pub status: StrategyStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_run: Option<DateTime<Utc>>,
    pub parameters: StrategyParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyLanguage {
    Python,
    Rust,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StrategyStatus {
    Idle,
    Running,
    Paused,
    Stopped,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyParameters {
    pub symbols: Vec<String>,
    pub timeframe: String,
    pub lookback_period: i64,
    pub risk_percentage: f64,
    pub max_positions: usize,
    pub custom_params: serde_json::Value,
}

impl Default for StrategyParameters {
    fn default() -> Self {
        Self {
            symbols: vec!["BTC/USD".to_string()],
            timeframe: "1h".to_string(),
            lookback_period: 100,
            risk_percentage: 1.0,
            max_positions: 5,
            custom_params: serde_json::json!({}),
        }
    }
}

impl Strategy {
    pub fn new(name: String, code: String, language: StrategyLanguage) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            code,
            language,
            status: StrategyStatus::Idle,
            created_at: now,
            updated_at: now,
            last_run: None,
            parameters: StrategyParameters::default(),
        }
    }
}