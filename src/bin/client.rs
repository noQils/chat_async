use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    // Continuous loop to handle both user input and server messages
    loop {
        tokio::select! {
            // Handle user input from stdin
            line = stdin.next_line() => {
                if let Some(line) = line? {
                    // Send the user's message to the server
                    ws_stream.send(Message::text(line)).await?;
                }
            }

            // Handle incoming messages from the server
            msg = ws_stream.next() => {
                if let Some(msg) = msg {
                    let msg = msg?;
                    if let Some(text) = msg.as_text() {
                        // Display the received message
                        println!("{}", text);
                    }
                }
            }
        }
    }
}