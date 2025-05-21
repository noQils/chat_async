use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Subscribe to the broadcast channel
    let mut bcast_rx = bcast_tx.subscribe();

    // Continuous loop to handle both incoming and outgoing messages
    loop {
        tokio::select! {
            // Handle incoming messages from the client
            Some(msg) = ws_stream.next() => {
                let msg = msg?;
                if let Some(text) = msg.as_text() {
                    // Broadcast the received message to all clients
                    let _ = bcast_tx.send(text.to_string());
                }
            }

            // Handle outgoing messages from the broadcast channel
            Ok(msg) = bcast_rx.recv() => {
                // Send the broadcast message to this client
                ws_stream.send(Message::text(msg)).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}