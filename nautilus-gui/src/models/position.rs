use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: Uuid,
    pub symbol: String,
    pub side: PositionSide,
    pub quantity: Decimal,
    pub entry_price: Decimal,
    pub current_price: Decimal,
    pub unrealized_pnl: Decimal,
    pub realized_pnl: Decimal,
    pub opened_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub stop_loss: Option<Decimal>,
    pub take_profit: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PositionSide {
    Long,
    Short,
}

impl Position {
    pub fn new(
        symbol: String,
        side: PositionSide,
        quantity: Decimal,
        entry_price: Decimal,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            symbol,
            side,
            quantity,
            entry_price,
            current_price: entry_price,
            unrealized_pnl: Decimal::ZERO,
            realized_pnl: Decimal::ZERO,
            opened_at: now,
            updated_at: now,
            stop_loss: None,
            take_profit: None,
        }
    }

    pub fn update_price(&mut self, price: Decimal) {
        self.current_price = price;
        self.unrealized_pnl = self.calculate_unrealized_pnl();
        self.updated_at = Utc::now();
    }

    pub fn calculate_unrealized_pnl(&self) -> Decimal {
        let price_diff = self.current_price - self.entry_price;
        match self.side {
            PositionSide::Long => price_diff * self.quantity,
            PositionSide::Short => -price_diff * self.quantity,
        }
    }

    pub fn calculate_return_percentage(&self) -> Decimal {
        let pnl_percentage = (self.current_price - self.entry_price) / self.entry_price * Decimal::from(100);
        match self.side {
            PositionSide::Long => pnl_percentage,
            PositionSide::Short => -pnl_percentage,
        }
    }
}