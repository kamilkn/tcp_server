use tokio::net::TcpListener;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use rand::Rng;
use sha2::{Sha256, Digest};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8084").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (random_string, challenge) = generate_challenge();
            socket.write_all(challenge.as_bytes()).await.unwrap();
            let mut buf = [0; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            let nonce = String::from_utf8_lossy(&buf[..n]).to_string();
            if verify_proof(&random_string, &nonce) {
                let quote = get_random_quote();  // Assume this function is implemented
                socket.write_all(quote.as_bytes()).await.unwrap();
            }
        });
    }
}


fn generate_challenge() -> (String, String) {
    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let challenge = format!("{}:{}", random_string, "00000");  // 5 leading zeros as an example
    (random_string, challenge)
}

fn verify_proof(random_string: &str, nonce: &str) -> bool {
    let data = format!("{}{}", random_string, nonce);
    let hash = Sha256::digest(data.as_bytes());
    hash.starts_with(b"00000")  // Assuming the requirement is 5 leading zeros
}

fn get_random_quote() -> String {
  "The only true wisdom is in knowing you know nothing.".to_string()  // Placeholder
}
