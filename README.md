# zk_square_proof

A minimal Rust project demonstrating a zero-knowledge proof (zk-SNARK) using [arkworks](https://github.com/arkworks-rs) libraries.  
This project proves knowledge of a secret number `x` such that `x * x = y`, without revealing `x` itself.

---

## Features

- Written in Rust, using the [arkworks](https://github.com/arkworks-rs) ecosystem
- Implements a basic zk-SNARK circuit for proving knowledge of a square root
- Uses the Groth16 proof system over the BN254 elliptic curve

## Zero-Knowledge Proofs (ZKPs) Overview

A zero-knowledge proof allows a prover to convince a verifier that they know a value (or that a statement is true) **without revealing any information** about the value itself.  
zk-SNARKs (Succinct Non-interactive ARguments of Knowledge) are efficient, small, and can be verified quickly, making them ideal for blockchain and privacy-preserving applications[4][5][6].

---

## How It Works

1. **Circuit Definition:**  
   The circuit enforces the constraint `y = x * x`, where `x` is private and `y` is public.

2. **Setup:**  
   Generates proving and verifying keys for the circuit (trusted setup).

3. **Proof Generation:**  
   Proves knowledge of `x` such that `x * x = y` without revealing `x`.

4. **Verification:**  
   Verifies the proof using only the public output `y`.

---
### Build and Run


git clone https://github.com/irajgill/zk_square_proof.git

cd zk_square_proof

cargo build

cargo run


You should see output similar to: Proof is valid: true

