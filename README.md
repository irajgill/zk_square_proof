# zk_square_proof

A minimal Rust project demonstrating a zero-knowledge proof (zk-SNARK) using [arkworks](https://github.com/arkworks-rs) libraries.  
This project proves knowledge of a secret number `x` such that `x * x = y`, without revealing `x` itself.

---

## Features

- Written in Rust, using the [arkworks](https://github.com/arkworks-rs) ecosystem
- Implements a basic zk-SNARK circuit for proving knowledge of a square root
- Uses the Groth16 proof system over the BN254 elliptic curve

---
### Build and Run

git clone https://github.com/irajgill/zk_square_proof.git
cd zk_square_proof
cargo build
cargo run


You should see output similar to: Proof is valid: true

