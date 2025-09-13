use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub event: String,
    pub symbol: String,
    pub data: serde_json::Value,
}

pub struct WebSocketClient {
    url: String,
    tx: mpsc::Sender<WebSocketMessage>,
}

impl WebSocketClient {
    pub fn new(url: String, tx: mpsc::Sender<WebSocketMessage>) -> Self {
        Self { url, tx }
    }
    
    pub async fn connect(&self) -> Result<()> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (mut write, mut read) = ws_stream.split();
        
        let tx = self.tx.clone();
        
        // Spawn task to handle incoming messages
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage>(&text) {
                            let _ = tx.send(ws_msg).await;
                        }
                    }
                    Ok(Message::Binary(bin)) => {
                        if let Ok(ws_msg) = serde_json::from_slice::<WebSocketMessage>(&bin) {
                            let _ = tx.send(ws_msg).await;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        tracing::info!("WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        tracing::error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });
        
        Ok(())
    }
    
    pub async fn subscribe(&self, symbols: Vec<String>) -> Result<()> {
        let subscribe_msg = serde_json::json!({
            "action": "subscribe",
            "symbols": symbols,
        });
        
        // In a real implementation, we'd send this through the WebSocket connection
        tracing::info!("Subscribing to symbols: {:?}", symbols);
        
        Ok(())
    }
}