//! # elnemer-core
//! Sovereign High-Performance AI Swarm, Cryptographic Proofs & Arabic NLP Kernel
//! Architected by Eng. Mahmoud Bek ElNemer (@ElNeMeR70)
//! Website: https://elnemer-ai.com

pub mod agent;
pub mod arabic;
pub mod crypto;

pub use agent::{Agent, Swarm, TaskResult};
pub use arabic::ArabicNLP;
pub use crypto::MerkleProof;
