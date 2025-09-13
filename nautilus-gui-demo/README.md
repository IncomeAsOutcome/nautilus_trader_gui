# NautilusTrader GUI Demo

A modern graphical user interface for NautilusTrader, showcasing the integration of Iced (retained mode) for the main UI and eGUI (immediate mode) for charting capabilities.

## 🚀 Features Demonstrated

- **Modern Dark Theme UI**: Professional trading interface with a sleek dark theme
- **Multi-View Navigation**: Dashboard, Trading, Backtesting, and Settings views
- **Real-time Data Display**: Mock market data and position tracking
- **Responsive Layout**: Sidebar navigation with main content area
- **Trading Components**: Order panels, position tables, and market overview

## 🏗️ Architecture

```
GUI Architecture
├── Iced Framework (Main UI)
│   ├── Retained mode rendering
│   ├── Component-based architecture
│   ├── Event-driven updates
│   └── Responsive layouts
│
├── eGUI Integration (Charts)
│   ├── Immediate mode rendering
│   ├── Real-time chart updates
│   ├── Technical indicators
│   └── Interactive controls
│
└── Backend Integration
    ├── PostgreSQL + TimescaleDB
    ├── NautilusTrader Core
    └── Python Strategy Engine
```

## 🎯 Key Components

### 1. Dashboard View
- Account balance and P&L tracking
- Active positions table
- Market overview with real-time prices
- Performance metrics

### 2. Trading View
- Price chart area (eGUI integration point)
- Order panel with buy/sell functionality
- Symbol selection and timeframe controls
- Order type selection (Market/Limit)

### 3. Backtesting View
- Strategy configuration panel
- Date range selection
- Backtest results display
- Performance metrics (Sharpe ratio, drawdown, etc.)

### 4. Settings View
- API configuration
- Database connection settings
- Theme preferences
- Trading parameters

## 🔧 Running the Demo

```bash
# Build the demo
cargo build --release

# Run the demo
cargo run --release
```

## 📦 Dependencies

- **Iced**: Cross-platform GUI framework
- **eGUI**: Immediate mode GUI for charts
- **rust_decimal**: Precise decimal arithmetic for financial calculations
- **chrono**: Date and time handling
- **tokio**: Async runtime for background tasks

## 🗄️ Database Setup

The full implementation uses PostgreSQL with TimescaleDB:

```bash
# Run with Docker
docker-compose up -d

# Database will be available at:
# postgresql://postgres:password@localhost:5432/nautilus_trader
```

## 🐍 Python Integration

The GUI integrates with NautilusTrader's Python strategies through PyO3:

```python
from nautilus_trader.trading.strategy import Strategy

class SimpleSMAStrategy(Strategy):
    def on_bar(self, bar):
        # Strategy logic here
        pass
```

## 📊 Data Sources

Supported data providers:
- **Mock Provider**: For testing and development
- **Alpha Vantage**: Free tier for market data
- **Binance**: Cryptocurrency data via WebSocket
- **Yahoo Finance**: Stock market data

## 🎨 UI Features

- **Dark Theme**: Optimized for extended trading sessions
- **Responsive Design**: Adapts to different screen sizes
- **Real-time Updates**: Live data refresh
- **Interactive Charts**: Zoom, pan, and technical indicators
- **Status Indicators**: Connection status and market state

## 🚧 Future Enhancements

- [ ] Full eGUI chart integration with real-time data
- [ ] WebSocket connections for live market data
- [ ] Advanced order types (Stop-loss, Take-profit)
- [ ] Multi-window support for multiple charts
- [ ] Strategy performance analytics
- [ ] Risk management dashboard
- [ ] Alert system for price movements
- [ ] Cloud synchronization

## 📝 Notes

This demo showcases the GUI capabilities and architecture. The full implementation would include:

1. Complete integration with NautilusTrader core
2. Real-time data feeds from exchanges
3. Live trading execution
4. Comprehensive backtesting engine
5. Python strategy execution environment
6. Advanced charting with technical indicators
7. Portfolio analytics and reporting

## 🤝 Contributing

This is a demonstration project showing how to build a modern trading GUI with Rust. The architecture can be extended to create a full-featured trading platform.

## 📄 License

LGPL-3.0 (matching NautilusTrader's license)