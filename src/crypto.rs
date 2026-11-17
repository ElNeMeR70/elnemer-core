//! Cryptographic verification and Merkle tree leaf hashing
use sha2::{Digest, Sha256};

pub struct MerkleProof;

impl MerkleProof {
    /// Computes SHA-256 hash of arbitrary bytes
    pub fn sha256_hex(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Computes Merkle parent node hash from left and right children
    pub fn combine_hashes(left: &str, right: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(left.as_bytes());
        hasher.update(right.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
