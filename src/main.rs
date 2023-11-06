use std::error::Error;
use std::env;
use dotenv::dotenv;
use log::{error, info};
use simple_logger::SimpleLogger;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};

use utils::sha2_256::{generate_challenge, verify_proof};

mod utils {
  pub mod sha2_256;
}

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
  dotenv().ok();

  let zeros: usize = env::var("ZEROS").unwrap().parse().expect("ZEROS must be a number");
  let length: usize = env::var("LENGTH").unwrap().parse().expect("LENGTH must be a number");

  let (random_string, challenge) = generate_challenge(length, zeros);

  info!("Challenge: {}", challenge);

  socket.write_all(challenge.as_bytes()).await?;

  let nonce = read_nonce(&mut socket).await?;

  if verify_proof(&random_string, &nonce, zeros) {
      info!("PoW verified");
      let quote = "Good job!";
      socket.write_all(quote.as_bytes()).await?;
  } else {
      error!("PoW verification failed");
  }

  Ok(())
}

pub async fn read_nonce(socket: &mut TcpStream) -> io::Result<String> {
  let mut buf = [0; 1024];
  let n = socket.read(&mut buf).await?;
  Ok(String::from_utf8_lossy(&buf[..n]).to_string())
}
