-- TimescaleDB hypertable for OHLCV candles
CREATE TABLE IF NOT EXISTS ohlcv_candles (
    instrument_id TEXT NOT NULL,
    timeframe TEXT NOT NULL, -- e.g. 1m,5m,1h,1d
    ts TIMESTAMPTZ NOT NULL,
    open DOUBLE PRECISION NOT NULL,
    high DOUBLE PRECISION NOT NULL,
    low DOUBLE PRECISION NOT NULL,
    close DOUBLE PRECISION NOT NULL,
    volume DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (instrument_id, timeframe, ts)
);

-- Create hypertable
SELECT create_hypertable('ohlcv_candles', 'ts', if_not_exists => TRUE);

-- Helpful index for range queries per instrument/timeframe
CREATE INDEX IF NOT EXISTS idx_ohlcv_instrument_timeframe_ts ON ohlcv_candles (instrument_id, timeframe, ts DESC);

