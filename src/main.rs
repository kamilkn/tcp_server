use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use rand::Rng;
use sha2::{Sha256, Digest};
use log::log;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8084").await?;
    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            match handle_connection(socket).await {
                Ok(_) => println!("Connection handled successfully"),
                Err(e) => eprintln!("Error handling connection: {:?}", e),
            }
        });

    }
}

async fn handle_connection(mut socket: TcpStream) -> io::Result<()> {
  let (random_string, challenge) = generate_challenge();
  println!("Challenge: {}", challenge);
  

  socket.write_all(challenge.as_bytes()).await?;

  let nonce = read_nonce(&mut socket).await?;
  
  if verify_proof(&random_string, &nonce) {
      println!("PoW verified");
      let quote = get_random_quote();
      socket.write_all(quote.as_bytes()).await?;
  } else {
      println!("PoW verification failed");
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
    let challenge = format!("{}:{}", random_string, "000");  // 3 leading zeros as an example
    (random_string, challenge)
}

fn verify_proof(random_string: &str, nonce: &str) -> bool {
    let data = format!("{}{}", random_string, nonce);
    let hash = Sha256::digest(data.as_bytes());
    hash.starts_with(b"000")  // Assuming the requirement is 3 leading zeros
}

fn get_random_quote() -> String {
  "The only true wisdom is in knowing you know nothing.".to_string()  // Placeholder
}
