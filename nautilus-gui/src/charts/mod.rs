use eframe::egui::{self, Context, Ui};
use egui_plot::{Bar, BarChart, Legend, Line, Plot};
use iced::{Element, Length};
use crate::app::Message;
use crate::models::MarketData;
use rust_decimal::prelude::*;

pub struct ChartView {
    symbol: String,
    data: Vec<MarketData>,
    zoom_level: f32,
    pan_offset: f32,
    show_volume: bool,
    show_indicators: bool,
}

impl ChartView {
    pub fn new() -> Self {
        Self {
            symbol: "BTC/USD".to_string(),
            data: Vec::new(),
            zoom_level: 1.0,
            pan_offset: 0.0,
            show_volume: true,
            show_indicators: false,
        }
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn update_data(&mut self, data: &[MarketData]) {
        self.data = data.to_vec();
    }

    pub fn zoom_in(&mut self) {
        self.zoom_level *= 1.2;
    }

    pub fn zoom_out(&mut self) {
        self.zoom_level /= 1.2;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = 0.0;
    }

    pub fn view(&self) -> Element<Message> {
        // For now, we'll create a placeholder canvas
        // In a real implementation, we'd integrate eGUI rendering here
        iced::widget::container(
            iced::widget::column![
                iced::widget::text(format!("Chart: {}", self.symbol)).size(16),
                iced::widget::container(
                    iced::widget::text("Chart will be rendered here with eGUI")
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20)
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    // This would be called in an eGUI context
    pub fn render_egui(&self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_chart(ui);
        });
    }

    fn render_chart(&self, ui: &mut Ui) {
        let plot = Plot::new("price_chart")
            .legend(Legend::default())
            .allow_zoom(true)
            .allow_drag(true)
            .allow_scroll(true);

        plot.show(ui, |plot_ui| {
            if !self.data.is_empty() {
                // Render candlestick chart
                self.render_candlesticks(plot_ui);
                
                // Render volume bars if enabled
                if self.show_volume {
                    self.render_volume_bars(plot_ui);
                }
                
                // Render indicators if enabled
                if self.show_indicators {
                    self.render_indicators(plot_ui);
                }
            }
        });
    }

    fn render_candlesticks(&self, plot_ui: &mut egui_plot::PlotUi) {
        // Convert market data to candlestick representation
        let mut high_points = Vec::new();
        let mut low_points = Vec::new();
        let mut open_points = Vec::new();
        let mut close_points = Vec::new();

        for (i, candle) in self.data.iter().enumerate() {
            let x = i as f64;
            high_points.push([x, candle.high.to_f64().unwrap_or(0.0)]);
            low_points.push([x, candle.low.to_f64().unwrap_or(0.0)]);
            open_points.push([x, candle.open.to_f64().unwrap_or(0.0)]);
            close_points.push([x, candle.close.to_f64().unwrap_or(0.0)]);
        }

        // Draw high-low lines
        for i in 0..self.data.len() {
            let line = Line::new(vec![
                [i as f64, low_points[i][1]],
                [i as f64, high_points[i][1]],
            ])
            .width(1.0);
            plot_ui.line(line);
        }

        // Draw open-close boxes
        for (i, candle) in self.data.iter().enumerate() {
            let color = if candle.close > candle.open {
                egui::Color32::GREEN
            } else {
                egui::Color32::RED
            };

            let open = candle.open.to_f64().unwrap_or(0.0);
            let close = candle.close.to_f64().unwrap_or(0.0);
            
            // Create a box for the candle body
            let body = Line::new(vec![
                [i as f64 - 0.3, open],
                [i as f64 + 0.3, open],
                [i as f64 + 0.3, close],
                [i as f64 - 0.3, close],
                [i as f64 - 0.3, open],
            ])
            .color(color)
            .width(2.0);
            
            plot_ui.line(body);
        }
    }

    fn render_volume_bars(&self, plot_ui: &mut egui_plot::PlotUi) {
        let bars: Vec<Bar> = self.data.iter().enumerate()
            .map(|(i, candle)| {
                Bar::new(i as f64, candle.volume.to_f64().unwrap_or(0.0))
                    .width(0.6)
            })
            .collect();

        let chart = BarChart::new(bars)
            .color(egui::Color32::from_rgba_premultiplied(100, 150, 200, 100));
        
        plot_ui.bar_chart(chart);
    }

    fn render_indicators(&self, plot_ui: &mut egui_plot::PlotUi) {
        // Simple Moving Average (SMA)
        if self.data.len() >= 20 {
            let sma_points = self.calculate_sma(20);
            let sma_line = Line::new(sma_points)
                .color(egui::Color32::YELLOW)
                .width(2.0)
                .name("SMA 20");
            plot_ui.line(sma_line);
        }

        // Add more indicators as needed
    }

    fn calculate_sma(&self, period: usize) -> Vec<[f64; 2]> {
        let mut sma_points = Vec::new();
        
        for i in period..self.data.len() {
            let sum: Decimal = self.data[i - period..i]
                .iter()
                .map(|d| d.close)
                .sum();
            let avg = sum / Decimal::from(period);
            sma_points.push([i as f64, avg.to_f64().unwrap_or(0.0)]);
        }
        
        sma_points
    }
}