use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Sandbox, Settings, Theme};

fn main() -> iced::Result {
    NautilusApp::run(Settings::default())
}

#[derive(Default)]
struct NautilusApp {
    open_chart_requested: bool,
}

#[derive(Debug, Clone)]
enum Message {
    OpenChartClicked,
}

impl Sandbox for NautilusApp {
    type Message = Message;

    fn new() -> Self { Self::default() }

    fn title(&self) -> String { "NautilusTrader GUI".into() }

    fn theme(&self) -> Theme { Theme::Dark }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OpenChartClicked => {
                self.open_chart_requested = true;
                std::thread::spawn(|| {
                    open_egui_chart_window();
                });
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let header = text("NautilusTrader with a beautiful, modern GUI");

        let open_chart = button("Open Chart Window (egui)")
            .on_press(Message::OpenChartClicked);

        let content = column![header, row![open_chart].spacing(16)]
            .spacing(24)
            .align_items(Alignment::Center);

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn open_egui_chart_window() {
    // Minimal eframe app launching egui window with a placeholder chart area
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Chart - egui",
        options,
        Box::new(|_cc| -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(Box::new(ChartApp::default()))
        }),
    );
}

#[derive(Default)]
struct ChartApp;

impl eframe::App for ChartApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.heading("Price Chart (MVP)");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            // Draw a simple placeholder line as a mock chart
            let rect = response.rect;
            let left = rect.left_top();
            let right = rect.right_bottom();
            painter.line_segment(
                [left, right],
                egui::Stroke { width: 2.0, color: egui::Color32::LIGHT_GREEN },
            );
        });
        ctx.request_repaint();
    }
}

