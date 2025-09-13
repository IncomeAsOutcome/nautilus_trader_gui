use nautilus_gui::NautilusApp;

fn main() {
    println!("Starting NautilusTrader GUI example...");
    
    // Run the application
    if let Err(e) = iced::application(
        NautilusApp::title,
        NautilusApp::update,
        NautilusApp::view
    )
    .theme(NautilusApp::theme)
    .run_with(NautilusApp::new) {
        eprintln!("Error running application: {}", e);
    }
}