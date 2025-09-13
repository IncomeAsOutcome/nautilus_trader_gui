-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;

-- Create database if not exists
-- Note: This is handled by docker-compose environment variables

-- Grant all privileges
GRANT ALL PRIVILEGES ON DATABASE nautilus_trader TO postgres;

-- Create initial schema
CREATE SCHEMA IF NOT EXISTS trading;

-- Set search path
SET search_path TO trading, public;

-- Add any initial data or configurations here
-- For example, create a default user or load sample data

-- Log successful initialization
DO $$
BEGIN
    RAISE NOTICE 'NautilusTrader database initialized successfully with TimescaleDB';
END $$;