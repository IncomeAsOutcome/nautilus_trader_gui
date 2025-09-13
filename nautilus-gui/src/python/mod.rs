use iced::{
    widget::{button, column, container, row, scrollable, text},
    Element, Length,
};
use crate::app::Message;

pub struct PythonEditor {
    code: String,
    output: String,
    is_running: bool,
}

impl PythonEditor {
    pub fn new() -> Self {
        Self {
            code: Self::default_strategy_code(),
            output: String::new(),
            is_running: false,
        }
    }

    pub fn set_code(&mut self, code: String) {
        self.code = code;
    }

    pub fn view(&self) -> Element<Message> {
        column![
            // Toolbar
            row![
                button("▶ Run").on_press(Message::RunStrategy),
                button("⏹ Stop").on_press(Message::StopStrategy),
                button("💾 Save"),
                button("📁 Load"),
            ]
            .spacing(10),
            
            // Editor and output split view
            row![
                // Code editor
                container(
                    column![
                        text("Strategy Code").size(16),
                        container(
                            scrollable(
                                text(&self.code).size(14)
                            )
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(10)
                    ]
                    .spacing(10)
                )
                .width(Length::FillPortion(2))
                .height(Length::Fill)
                .padding(10),
                
                // Output console
                container(
                    column![
                        text("Output").size(16),
                        container(
                            scrollable(
                                text(&self.output).size(12)
                            )
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(10)
                    ]
                    .spacing(10)
                )
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .padding(10),
            ]
            .spacing(20)
            .height(Length::Fill)
        ]
        .spacing(10)
        .into()
    }

    fn default_strategy_code() -> String {
        r#"from nautilus_trader.trading.strategy import Strategy
from nautilus_trader.model.identifiers import InstrumentId
from nautilus_trader.model.data import Bar, BarType
from nautilus_trader.model.enums import OrderSide
from nautilus_trader.indicators.average.sma import SimpleMovingAverage

class SimpleSMAStrategy(Strategy):
    """
    A simple moving average crossover strategy.
    """
    
    def __init__(self, config):
        super().__init__(config)
        
        # Configuration
        self.instrument_id = InstrumentId.from_str(config.instrument_id)
        self.bar_type = BarType.from_str(config.bar_type)
        self.fast_period = config.fast_period
        self.slow_period = config.slow_period
        self.trade_size = config.trade_size
        
        # Indicators
        self.fast_sma = SimpleMovingAverage(self.fast_period)
        self.slow_sma = SimpleMovingAverage(self.slow_period)
        
        # State
        self.position = None
        
    def on_start(self):
        """Called when the strategy is started."""
        self.subscribe_bars(self.bar_type)
        
    def on_bar(self, bar: Bar):
        """Called when a new bar is received."""
        # Update indicators
        self.fast_sma.update_raw(bar.close.as_double())
        self.slow_sma.update_raw(bar.close.as_double())
        
        # Check if indicators are ready
        if not self.fast_sma.initialized or not self.slow_sma.initialized:
            return
            
        # Get current values
        fast_value = self.fast_sma.value
        slow_value = self.slow_sma.value
        
        # Trading logic
        if self.position is None:
            # Check for entry signals
            if fast_value > slow_value:
                self.buy()
            elif fast_value < slow_value:
                self.sell()
        else:
            # Check for exit signals
            if self.position.is_long and fast_value < slow_value:
                self.close_position()
                self.sell()
            elif self.position.is_short and fast_value > slow_value:
                self.close_position()
                self.buy()
                
    def buy(self):
        """Submit a buy order."""
        order = self.order_factory.market(
            instrument_id=self.instrument_id,
            order_side=OrderSide.BUY,
            quantity=self.trade_size,
        )
        self.submit_order(order)
        
    def sell(self):
        """Submit a sell order."""
        order = self.order_factory.market(
            instrument_id=self.instrument_id,
            order_side=OrderSide.SELL,
            quantity=self.trade_size,
        )
        self.submit_order(order)
        
    def close_position(self):
        """Close the current position."""
        if self.position:
            self.close_position(self.position)
            
    def on_stop(self):
        """Called when the strategy is stopped."""
        self.cancel_all_orders(self.instrument_id)
        self.close_all_positions(self.instrument_id)
"#.to_string()
    }
}

// Python execution integration
pub mod executor {
    use pyo3::prelude::*;
    use pyo3::types::PyDict;
    
    pub struct PythonExecutor {
        interpreter: Option<Python<'static>>,
    }
    
    impl PythonExecutor {
        pub fn new() -> Self {
            Self {
                interpreter: None,
            }
        }
        
        pub fn execute_strategy(&self, code: &str) -> Result<String, String> {
            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                
                match py.run(code, None, Some(locals)) {
                    Ok(_) => {
                        // Get output from locals if available
                        Ok("Strategy executed successfully".to_string())
                    }
                    Err(e) => {
                        Err(format!("Error executing strategy: {}", e))
                    }
                }
            })
        }
    }
}