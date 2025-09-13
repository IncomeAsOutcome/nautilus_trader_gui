"""
Simple Moving Average Crossover Strategy for NautilusTrader GUI
"""

from decimal import Decimal
from nautilus_trader.config import StrategyConfig
from nautilus_trader.core.data import Data
from nautilus_trader.model.data import Bar, BarType
from nautilus_trader.model.enums import OrderSide, TimeInForce
from nautilus_trader.model.identifiers import InstrumentId
from nautilus_trader.model.instruments import Instrument
from nautilus_trader.model.orders import MarketOrder
from nautilus_trader.trading.strategy import Strategy


class SMAConfig(StrategyConfig):
    """Configuration for the SMA crossover strategy."""
    
    instrument_id: str = "BTC/USD"
    bar_type: str = "BTC/USD-1-HOUR-BID-INTERNAL"
    fast_period: int = 10
    slow_period: int = 20
    trade_size: Decimal = Decimal("0.01")
    

class SMACrossStrategy(Strategy):
    """
    A simple moving average crossover strategy.
    
    When the fast SMA crosses above the slow SMA, enter long.
    When the fast SMA crosses below the slow SMA, enter short.
    """
    
    def __init__(self, config: SMAConfig) -> None:
        super().__init__(config)
        
        # Configuration
        self.instrument_id = InstrumentId.from_str(config.instrument_id)
        self.bar_type = BarType.from_str(config.bar_type)
        self.fast_period = config.fast_period
        self.slow_period = config.slow_period
        self.trade_size = config.trade_size
        
        # State
        self.fast_sma = []
        self.slow_sma = []
        self.position_side = None
        
    def on_start(self) -> None:
        """Actions to be performed on strategy start."""
        self.log.info("Starting SMA Crossover Strategy")
        
        # Subscribe to bar data
        self.subscribe_bars(self.bar_type)
        
    def on_bar(self, bar: Bar) -> None:
        """Actions to be performed when a bar is received."""
        
        # Update price history
        close_price = float(bar.close)
        
        # Calculate SMAs
        self.fast_sma.append(close_price)
        self.slow_sma.append(close_price)
        
        # Keep only required periods
        if len(self.fast_sma) > self.fast_period:
            self.fast_sma.pop(0)
        if len(self.slow_sma) > self.slow_period:
            self.slow_sma.pop(0)
            
        # Check if we have enough data
        if len(self.fast_sma) < self.fast_period or len(self.slow_sma) < self.slow_period:
            return
            
        # Calculate current SMA values
        fast_value = sum(self.fast_sma[-self.fast_period:]) / self.fast_period
        slow_value = sum(self.slow_sma[-self.slow_period:]) / self.slow_period
        
        # Log current values
        self.log.info(
            f"Bar: {bar.close} | Fast SMA: {fast_value:.2f} | Slow SMA: {slow_value:.2f}"
        )
        
        # Generate trading signals
        if fast_value > slow_value and self.position_side != "long":
            self._go_long()
        elif fast_value < slow_value and self.position_side != "short":
            self._go_short()
            
    def _go_long(self) -> None:
        """Enter or flip to long position."""
        # Close short position if exists
        if self.position_side == "short":
            self._close_position()
            
        # Enter long position
        order = self.order_factory.market(
            instrument_id=self.instrument_id,
            order_side=OrderSide.BUY,
            quantity=self.trade_size,
            time_in_force=TimeInForce.IOC,
        )
        
        self.submit_order(order)
        self.position_side = "long"
        self.log.info("Entering LONG position")
        
    def _go_short(self) -> None:
        """Enter or flip to short position."""
        # Close long position if exists
        if self.position_side == "long":
            self._close_position()
            
        # Enter short position
        order = self.order_factory.market(
            instrument_id=self.instrument_id,
            order_side=OrderSide.SELL,
            quantity=self.trade_size,
            time_in_force=TimeInForce.IOC,
        )
        
        self.submit_order(order)
        self.position_side = "short"
        self.log.info("Entering SHORT position")
        
    def _close_position(self) -> None:
        """Close current position."""
        if self.position_side == "long":
            order = self.order_factory.market(
                instrument_id=self.instrument_id,
                order_side=OrderSide.SELL,
                quantity=self.trade_size,
                time_in_force=TimeInForce.IOC,
            )
            self.submit_order(order)
            
        elif self.position_side == "short":
            order = self.order_factory.market(
                instrument_id=self.instrument_id,
                order_side=OrderSide.BUY,
                quantity=self.trade_size,
                time_in_force=TimeInForce.IOC,
            )
            self.submit_order(order)
            
        self.position_side = None
        self.log.info("Position closed")
        
    def on_stop(self) -> None:
        """Actions to be performed on strategy stop."""
        # Close any open positions
        if self.position_side:
            self._close_position()
            
        self.log.info("Strategy stopped")
        
    def on_reset(self) -> None:
        """Actions to be performed on strategy reset."""
        self.fast_sma.clear()
        self.slow_sma.clear()
        self.position_side = None
        self.log.info("Strategy reset")