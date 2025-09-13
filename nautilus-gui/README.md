# NautilusTrader GUI

A modern, user-friendly graphical user interface for NautilusTrader, combining the power of Rust-based trading infrastructure with intuitive visual tools.

## Features

- **Modern UI/UX**: Built with Iced (retained mode) for the main interface and eGUI (immediate mode) for real-time charting
- **Integrated Python IDE**: Built-in Python editor for strategy development with syntax highlighting and execution
- **Real-time Charting**: Advanced charting capabilities with technical indicators, drawing tools, and multiple timeframes
- **Portfolio Management**: Track positions, orders, and P&L in real-time
- **Backtesting Interface**: Visual backtesting with performance metrics and trade analysis
- **Multi-Asset Support**: Trade crypto, forex, stocks, and futures from a single platform
- **Database Integration**: PostgreSQL with TimescaleDB for efficient time-series data storage

## Architecture

```
nautilus-gui/
├── src/
│   ├── app.rs           # Main application logic (Iced)
│   ├── charts/          # Charting components (eGUI)
│   ├── data/            # Data providers and WebSocket clients
│   ├── database/        # Database models and queries
│   ├── models/          # Domain models
│   ├── python/          # Python integration for strategies
│   ├── services/        # Business logic services
│   └── ui/              # UI components
```

## Technology Stack

- **Frontend**: Iced + eGUI (Rust)
- **Backend**: NautilusTrader core (Rust/Python)
- **Database**: PostgreSQL + TimescaleDB
- **Data Sources**: Alpha Vantage, Yahoo Finance, Binance (configurable)
- **Python Integration**: PyO3 for embedded Python execution

## Getting Started

### Prerequisites

- Rust 1.89.0 or later
- PostgreSQL 14+ with TimescaleDB extension
- Python 3.11+ (for strategy development)
- Docker and Docker Compose (optional, for database)

### Installation

1. **Clone the repository**:
```bash
git clone https://github.com/yourusername/nautilus-trader.git
cd nautilus-trader/nautilus-gui
```

2. **Set up the database**:
```bash
docker-compose up -d
```

3. **Build and run the GUI**:
```bash
cargo build --release
cargo run --release
```

### Configuration

Create a `config.toml` file in the project root:

```toml
[database]
url = "postgresql://postgres:password@localhost:5432/nautilus_trader"

[data]
provider = "mock"  # or "alpha_vantage", "binance"
# alpha_vantage_api_key = "YOUR_API_KEY"

[ui]
theme = "dark"
auto_save = true
```

## Usage

### Trading View

1. Select a symbol from the top bar
2. Choose your timeframe
3. Place orders using the order panel
4. Monitor positions in real-time

### Strategy Development

1. Navigate to "Strategy Dev" in the sidebar
2. Write your Python strategy in the integrated editor
3. Test with historical data
4. Deploy to live trading

### Backtesting

1. Go to "Backtesting" view
2. Select strategy and parameters
3. Choose date range and initial capital
4. Run backtest and analyze results

## Data Providers

### Mock Provider (Default)
- Generates synthetic data for testing
- No API key required
- Suitable for development and testing

### Alpha Vantage
- Free tier available (5 API calls/minute)
- Supports stocks, forex, and crypto
- Get API key at: https://www.alphavantage.co/support/#api-key

### Binance
- Real-time crypto data
- Requires API key and secret
- WebSocket support for live data

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=nautilus_gui=debug cargo run
```

### Project Structure

- **Models**: Domain objects (Order, Position, Strategy, etc.)
- **Services**: Business logic layer
- **UI Components**: Reusable UI elements
- **Data Providers**: Pluggable data source implementations
- **Database**: PostgreSQL integration with TimescaleDB

## Performance Optimization

- **Immediate Mode (eGUI)**: Used for charts requiring frequent updates
- **Retained Mode (Iced)**: Used for stable UI components
- **Async I/O**: Tokio runtime for non-blocking operations
- **Time-Series Optimization**: TimescaleDB for efficient data queries
- **Caching**: In-memory cache for frequently accessed data

## Roadmap

- [ ] Advanced charting tools (Fibonacci, trend lines, etc.)
- [ ] Multi-window support
- [ ] Strategy marketplace
- [ ] Cloud sync and backup
- [ ] Mobile companion app
- [ ] AI-powered strategy suggestions
- [ ] Social trading features
- [ ] Advanced risk management tools

## Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests to the `develop` branch.

## License

This project is licensed under the LGPL-3.0 License - see the LICENSE file for details.

## Support

- Documentation: https://nautilustrader.io/docs/
- Discord: https://discord.gg/NautilusTrader
- Issues: https://github.com/nautechsystems/nautilus_trader/issues

## Acknowledgments

- NautilusTrader team for the excellent trading platform
- Iced and eGUI communities for the GUI frameworks
- TimescaleDB for time-series database capabilities