mod simple_app;
use simple_app::SimpleApp;

fn main() {
    println!("Starting NautilusTrader GUI Demo...");
    
    // Run the application
    iced::application(SimpleApp::title, SimpleApp::update, SimpleApp::view)
        .theme(SimpleApp::theme)
        .run_with(SimpleApp::new)
        .expect("Failed to run application");
}