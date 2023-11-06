use super::ProofOfWork;
use rand::Rng;
use sha2::{Digest, Sha256};

pub struct Sha2_256;

impl ProofOfWork for Sha2_256 {
    fn generate_challenge(&self, length: usize, zeros: usize) -> (String, String) {
        let random_string: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
        let challenge_zeros = "0".repeat(zeros);
        let challenge = format!("{}:{}", random_string, challenge_zeros);
        (random_string, challenge)
    }

    fn verify_proof(&self, random_string: &str, nonce: &str, zeros: usize) -> bool {
        let data = format!("{}{}", random_string, nonce);
        let hash = Sha256::digest(data.as_bytes());
        hash.starts_with("0".repeat(zeros).as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_challenge() {
        let pow_algorithm = Sha2_256;
        let length = 10;
        let zeros = 4;
        let (random_string, challenge) =
            ProofOfWork::generate_challenge(&pow_algorithm, length, zeros);
        assert_eq!(random_string.len(), length);
        assert!(challenge.contains(&"0".repeat(zeros)));
    }

    #[test]
    fn test_verify_proof() {
        let pow_algorithm = Sha2_256;
        let test_cases = vec![
            ("random_string", "123", 4, false),
            ("CFc2hAsAk6", "6032407", 3, true),
            ("FWDvotNNpm", "634", 1, true),
            ("FWDvotNNpm", "634", 2, false),
            ("random_string", "123", 0, true),
        ];

        for (random_string, nonce, zeros, expected) in test_cases {
            assert_eq!(
                ProofOfWork::verify_proof(&pow_algorithm, random_string, nonce, zeros),
                expected,
                "Failed on random_string: '{}', nonce: '{}', zeros: {}",
                random_string,
                nonce,
                zeros
            );
        }
    }
}
