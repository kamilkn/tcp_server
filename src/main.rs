use log::{error, info};

use simple_logger::SimpleLogger;
use std::error::Error;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream}; 
mod config;
mod zeros_algorithm;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    match run().await {
        Ok(()) => info!("Program completed (exit from loop)"),
        Err(e) => error!("Error: {}", e),
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8084").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            match handle_connection(socket).await {
                Ok(_) => info!("Request completed"),
                Err(e) => error!("Error handling connection: {:?}", e),
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream) -> io::Result<()> {

  use config::{ZEROS, LENGTH};
  use zeros_algorithm::{read_nonce, generate_challenge, verify_proof};

  let (random_string, challenge) = generate_challenge(LENGTH, ZEROS);

  info!("Challenge: {}", challenge);

  socket.write_all(challenge.as_bytes()).await?;

  let nonce = read_nonce(&mut socket).await?;

  if verify_proof(&random_string, &nonce, ZEROS) {
      info!("PoW verified");
      let quote = "Good job!";
      socket.write_all(quote.as_bytes()).await?;
  } else {
      error!("PoW verification failed");
  }

  Ok(())
}
