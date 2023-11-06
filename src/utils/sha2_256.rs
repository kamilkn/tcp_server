use rand::Rng;
use sha2::{Digest, Sha256};

pub fn generate_challenge(length: usize, zeros: usize) -> (String, String) {
    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    let challenge_zeros = "0".repeat(zeros);
    let challenge = format!("{}:{}", random_string, challenge_zeros);
    (random_string, challenge)
}

pub fn verify_proof(random_string: &str, nonce: &str, zeros: usize) -> bool {
    let data = format!("{}{}", random_string, nonce);
    let hash = Sha256::digest(data.as_bytes());
    hash.starts_with("0".repeat(zeros).as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_challenge() {
        let length = 10;
        let zeros = 4;
        let (random_string, challenge) = generate_challenge(length, zeros);
        assert_eq!(random_string.len(), length);
        assert!(challenge.contains(&"0".repeat(zeros)));
    }

    #[test]
    fn test_verify_proof() {
        assert_eq!(verify_proof("random_string", "123", 4), false);
        assert_eq!(verify_proof("CFc2hAsAk6", "6032407", 3), true);
        assert_eq!(verify_proof("FWDvotNNpm", "634", 1), true);
        assert_eq!(verify_proof("FWDvotNNpm", "634", 2), false);
        assert_eq!(verify_proof("random_string", "123", 0), true);
    }
}
