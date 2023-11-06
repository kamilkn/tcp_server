use dotenv::dotenv;
use log::{error, info};
use simple_logger::SimpleLogger;
use std::env;
use std::error::Error;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use utils::{ProofOfWork, sha2_256::Sha2_256};
mod utils;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    dotenv().ok();

    match run().await {
        Ok(()) => info!("Program completed (exit from loop)"),
        Err(e) => error!("Error: {}", e),
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

    info!("Server run on {}:{}", host, port);

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
    let zeros: usize = env::var("ZEROS")
        .unwrap()
        .parse()
        .expect("ZEROS must be a number");
    let length: usize = env::var("LENGTH")
        .unwrap()
        .parse()
        .expect("LENGTH must be a number");

    let pow_algorithm = Sha2_256;

    let (random_string, challenge) = ProofOfWork::generate_challenge(&pow_algorithm, length, zeros);

    info!("Challenge: {}", challenge);

    socket.write_all(challenge.as_bytes()).await?;

    let nonce = read_nonce(&mut socket).await?;

    if Sha2_256::verify_proof(&pow_algorithm,&random_string, &nonce, zeros) {
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
