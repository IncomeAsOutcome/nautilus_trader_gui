use anyhow::Result;
use iced;

mod app;
mod charts;
mod data;
mod database;
mod models;
mod python;
mod services;
mod ui;

use app::NautilusApp;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("nautilus_gui=debug,iced=info,egui=info")
        .init();

    tracing::info!("Starting NautilusTrader GUI...");

    // Note: Database initialization should be done in the app's initialization
    // We'll use a simpler approach for now
    
    // Run the Iced application
    iced::application(NautilusApp::title, NautilusApp::update, NautilusApp::view)
        .theme(NautilusApp::theme)
        .run_with(NautilusApp::new)?;

    Ok(())
}