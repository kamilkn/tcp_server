pub mod sha2_256;

pub trait ProofOfWork {
  fn generate_challenge(&self, length: usize, zeros: usize) -> (String, String);
  fn verify_proof(&self, random_string: &str, nonce: &str, zeros: usize) -> bool;
}
