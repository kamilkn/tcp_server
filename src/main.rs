use log::{error, info};
use rand::Rng;
use sha2::{Digest, Sha256};
use simple_logger::SimpleLogger;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const REQUIRED_ZEROS: &str = "0000";

#[tokio::main]
async fn main() -> io::Result<()> {
    SimpleLogger::new().init().unwrap();
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
    let (random_string, challenge) = generate_challenge();

    info!("Challenge: {}", challenge);

    socket.write_all(challenge.as_bytes()).await?;

    let nonce = read_nonce(&mut socket).await?;

    if verify_proof(&random_string, &nonce) {
        info!("PoW verified");
        let quote = "Good job!";
        socket.write_all(quote.as_bytes()).await?;
    } else {
        error!("PoW verification failed");
    }

    Ok(())
}

async fn read_nonce(socket: &mut TcpStream) -> io::Result<String> {
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf[..n]).to_string())
}

fn generate_challenge() -> (String, String) {
    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let challenge = format!("{}:{}", random_string, REQUIRED_ZEROS); // 3 leading zeros as an example
    (random_string, challenge)
}

fn verify_proof(random_string: &str, nonce: &str) -> bool {
    let data = format!("{}{}", random_string, nonce);
    let hash = Sha256::digest(data.as_bytes());
    hash.starts_with(REQUIRED_ZEROS.as_bytes()) // Assuming the requirement is 3 leading zeros
}
