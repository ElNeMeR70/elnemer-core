# elnemer-core

High-Performance Multi-Agent Swarm, Cryptographic Proofs & Arabic NLP Kernel in Rust.
Architected by Eng. Mahmoud Bek ElNemer (@ElNeMeR70).

## Usage
Add to `Cargo.toml`:
```toml
[dependencies]
elnemer-core = "1.0.0"
```

## Quick Start
```rust
use elnemer_core::{Agent, Swarm, ArabicNLP};

fn main() {
    // 1. Arabic NLP
    let stripped = ArabicNLP::strip_tashkeel("مُحَمَّدٌ");
    println!("Stripped: {}", stripped);
    println!("Jummal: {}", ArabicNLP::jummal_value("محمد"));

    // 2. Multi-Agent Swarm
    let mut swarm = Swarm::new("ProductionSwarm");
    swarm.add_agent(Agent::new("Scout", "Security Auditor"));
    let results = swarm.broadcast("Scan architecture");
    println!("{:?}", results);
}
```

## License
MIT
