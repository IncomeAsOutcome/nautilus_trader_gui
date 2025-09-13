use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tokio::sync::RwLock;

pub static DATABASE_URL: &str = "postgresql://postgres:password@localhost:5432/nautilus_trader";

// We'll use once_cell instead of lazy_static for better compatibility
use once_cell::sync::Lazy;

static DB_POOL: Lazy<Arc<RwLock<Option<Pool<Postgres>>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(None))
});

pub async fn init() -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;
    
    // Run migrations
    create_tables(&pool).await?;
    
    let mut db = DB_POOL.write().await;
    *db = Some(pool);
    
    tracing::info!("Database initialized successfully");
    Ok(())
}

pub async fn get_pool() -> Result<Pool<Postgres>> {
    let db = DB_POOL.read().await;
    db.clone().ok_or_else(|| anyhow::anyhow!("Database not initialized"))
}

async fn create_tables(pool: &Pool<Postgres>) -> Result<()> {
    // Create TimescaleDB extension
    sqlx::query(
        r#"
        CREATE EXTENSION IF NOT EXISTS timescaledb;
        "#
    )
    .execute(pool)
    .await?;
    
    // Create market data table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS market_data (
            symbol VARCHAR(50) NOT NULL,
            timestamp TIMESTAMPTZ NOT NULL,
            open DECIMAL(20, 8) NOT NULL,
            high DECIMAL(20, 8) NOT NULL,
            low DECIMAL(20, 8) NOT NULL,
            close DECIMAL(20, 8) NOT NULL,
            volume DECIMAL(20, 8) NOT NULL,
            PRIMARY KEY (symbol, timestamp)
        );
        "#
    )
    .execute(pool)
    .await?;
    
    // Convert to hypertable for TimescaleDB
    sqlx::query(
        r#"
        SELECT create_hypertable('market_data', 'timestamp', 
            if_not_exists => TRUE,
            chunk_time_interval => INTERVAL '1 day'
        );
        "#
    )
    .execute(pool)
    .await
    .ok(); // Ignore error if already a hypertable
    
    // Create orders table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS orders (
            id UUID PRIMARY KEY,
            symbol VARCHAR(50) NOT NULL,
            side VARCHAR(10) NOT NULL,
            order_type VARCHAR(20) NOT NULL,
            quantity DECIMAL(20, 8) NOT NULL,
            price DECIMAL(20, 8),
            stop_price DECIMAL(20, 8),
            time_in_force VARCHAR(10) NOT NULL,
            status VARCHAR(20) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL,
            filled_quantity DECIMAL(20, 8) NOT NULL DEFAULT 0,
            average_fill_price DECIMAL(20, 8)
        );
        "#
    )
    .execute(pool)
    .await?;
    
    // Create positions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS positions (
            id UUID PRIMARY KEY,
            symbol VARCHAR(50) NOT NULL,
            side VARCHAR(10) NOT NULL,
            quantity DECIMAL(20, 8) NOT NULL,
            entry_price DECIMAL(20, 8) NOT NULL,
            current_price DECIMAL(20, 8) NOT NULL,
            unrealized_pnl DECIMAL(20, 8) NOT NULL,
            realized_pnl DECIMAL(20, 8) NOT NULL,
            opened_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL,
            stop_loss DECIMAL(20, 8),
            take_profit DECIMAL(20, 8)
        );
        "#
    )
    .execute(pool)
    .await?;
    
    // Create strategies table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS strategies (
            id UUID PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            code TEXT NOT NULL,
            language VARCHAR(20) NOT NULL,
            status VARCHAR(20) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL,
            last_run TIMESTAMPTZ,
            parameters JSONB NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;
    
    // Create backtests table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS backtests (
            id UUID PRIMARY KEY,
            strategy_id UUID REFERENCES strategies(id),
            start_date TIMESTAMPTZ NOT NULL,
            end_date TIMESTAMPTZ NOT NULL,
            initial_capital DECIMAL(20, 8) NOT NULL,
            final_capital DECIMAL(20, 8),
            total_return DECIMAL(10, 4),
            sharpe_ratio DECIMAL(10, 4),
            max_drawdown DECIMAL(10, 4),
            win_rate DECIMAL(10, 4),
            profit_factor DECIMAL(10, 4),
            total_trades INTEGER,
            created_at TIMESTAMPTZ NOT NULL,
            results JSONB
        );
        "#
    )
    .execute(pool)
    .await?;
    
    // Create indices for better performance
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_market_data_symbol_timestamp 
        ON market_data (symbol, timestamp DESC);
        
        CREATE INDEX IF NOT EXISTS idx_orders_symbol_status 
        ON orders (symbol, status);
        
        CREATE INDEX IF NOT EXISTS idx_positions_symbol 
        ON positions (symbol);
        "#
    )
    .execute(pool)
    .await?;
    
    tracing::info!("Database tables created successfully");
    Ok(())
}