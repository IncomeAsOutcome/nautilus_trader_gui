use iced::{
    widget::{button, column, container, row, text, Column, Container, Row},
    Element, Length, Task, Theme,
};
use crate::ui::{MainView, Sidebar, TopBar};
use crate::charts::ChartView;
use crate::python::PythonEditor;
use crate::models::AppState;

pub struct NautilusApp {
    state: AppState,
    main_view: MainView,
    sidebar: Sidebar,
    top_bar: TopBar,
    chart_view: ChartView,
    python_editor: PythonEditor,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    ViewChanged(ViewType),
    
    // Trading
    PlaceOrder,
    CancelOrder(String),
    
    // Data
    DataReceived(Vec<crate::models::MarketData>),
    SymbolSelected(String),
    
    // Python IDE
    CodeChanged(String),
    RunStrategy,
    StopStrategy,
    
    // Settings
    ThemeChanged(Theme),
    
    // Chart interactions
    ChartZoomIn,
    ChartZoomOut,
    ChartReset,
    TimeframeChanged(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViewType {
    Dashboard,
    Trading,
    Backtesting,
    StrategyDevelopment,
    Portfolio,
    Settings,
}

impl NautilusApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: AppState::default(),
                main_view: MainView::new(),
                sidebar: Sidebar::new(),
                top_bar: TopBar::new(),
                chart_view: ChartView::new(),
                python_editor: PythonEditor::new(),
                theme: Theme::Dark,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        String::from("NautilusTrader - Modern Trading Platform")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ViewChanged(view) => {
                self.state.current_view = view;
                Task::none()
            }
            Message::SymbolSelected(symbol) => {
                self.state.selected_symbol = Some(symbol.clone());
                self.chart_view.set_symbol(symbol);
                Task::none()
            }
            Message::DataReceived(data) => {
                self.state.market_data.extend(data);
                self.chart_view.update_data(&self.state.market_data);
                Task::none()
            }
            Message::CodeChanged(code) => {
                self.python_editor.set_code(code);
                Task::none()
            }
            Message::RunStrategy => {
                // TODO: Implement strategy execution
                tracing::info!("Running strategy...");
                Task::none()
            }
            Message::StopStrategy => {
                // TODO: Implement strategy stopping
                tracing::info!("Stopping strategy...");
                Task::none()
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::ChartZoomIn => {
                self.chart_view.zoom_in();
                Task::none()
            }
            Message::ChartZoomOut => {
                self.chart_view.zoom_out();
                Task::none()
            }
            Message::ChartReset => {
                self.chart_view.reset_view();
                Task::none()
            }
            Message::TimeframeChanged(timeframe) => {
                self.state.selected_timeframe = timeframe;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let sidebar = self.sidebar.view(&self.state);
        let top_bar = self.top_bar.view(&self.state);
        
        let main_content = match self.state.current_view {
            ViewType::Dashboard => self.main_view.dashboard_view(&self.state),
            ViewType::Trading => self.trading_view(),
            ViewType::Backtesting => self.backtesting_view(),
            ViewType::StrategyDevelopment => self.strategy_development_view(),
            ViewType::Portfolio => self.portfolio_view(),
            ViewType::Settings => self.settings_view(),
        };

        container(
            column![
                top_bar,
                row![
                    sidebar,
                    container(main_content)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(20)
                ]
                .width(Length::Fill)
                .height(Length::Fill)
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

impl NautilusApp {
    fn trading_view(&self) -> Element<Message> {
        column![
            text("Trading View").size(24),
            row![
                container(self.chart_view.view())
                    .width(Length::FillPortion(3))
                    .height(Length::Fill),
                container(self.order_panel())
                    .width(Length::FillPortion(1))
                    .height(Length::Fill)
                    .padding(10)
            ]
            .spacing(20)
        ]
        .spacing(20)
        .into()
    }

    fn backtesting_view(&self) -> Element<Message> {
        column![
            text("Backtesting").size(24),
            row![
                container(self.chart_view.view())
                    .width(Length::FillPortion(2))
                    .height(Length::Fill),
                container(self.backtest_controls())
                    .width(Length::FillPortion(1))
                    .height(Length::Fill)
                    .padding(10)
            ]
            .spacing(20)
        ]
        .spacing(20)
        .into()
    }

    fn strategy_development_view(&self) -> Element<Message> {
        column![
            text("Strategy Development").size(24),
            row![
                container(self.python_editor.view())
                    .width(Length::FillPortion(2))
                    .height(Length::Fill),
                container(self.strategy_controls())
                    .width(Length::FillPortion(1))
                    .height(Length::Fill)
                    .padding(10)
            ]
            .spacing(20)
        ]
        .spacing(20)
        .into()
    }

    fn portfolio_view(&self) -> Element<Message> {
        column![
            text("Portfolio Overview").size(24),
            text("Portfolio analytics and positions will be displayed here")
        ]
        .spacing(20)
        .into()
    }

    fn settings_view(&self) -> Element<Message> {
        column![
            text("Settings").size(24),
            text("Application settings and configurations")
        ]
        .spacing(20)
        .into()
    }

    fn order_panel(&self) -> Element<Message> {
        column![
            text("Order Panel").size(20),
            button("Buy").on_press(Message::PlaceOrder).width(Length::Fill),
            button("Sell").on_press(Message::PlaceOrder).width(Length::Fill),
        ]
        .spacing(10)
        .into()
    }

    fn backtest_controls(&self) -> Element<Message> {
        column![
            text("Backtest Controls").size(20),
            button("Run Backtest").on_press(Message::RunStrategy).width(Length::Fill),
            button("Stop").on_press(Message::StopStrategy).width(Length::Fill),
        ]
        .spacing(10)
        .into()
    }

    fn strategy_controls(&self) -> Element<Message> {
        column![
            text("Strategy Controls").size(20),
            button("Run Strategy").on_press(Message::RunStrategy).width(Length::Fill),
            button("Stop Strategy").on_press(Message::StopStrategy).width(Length::Fill),
        ]
        .spacing(10)
        .into()
    }
}