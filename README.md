# Shamir's secret sharing
Rust port of Shamir's secret sharing from https://github.com/blockstack/secret-sharing. Distribute arbitary secrets among multiple parties.

## Usage

Run example with 

```cargo run --example ss_example```

```rust
extern crate secretsharing;

use secretsharing::ss::{reconstruct_secret, Charset, SecretSharing};

fn main() {
    let threshold = 3;
    let total_shares = 5;
    let secret = "HelloWorld123";
    let mut ss = SecretSharing::new(
        threshold, // Minimum shares required for reconstruction.
        total_shares, // Total shares.
        Charset::Alphanumeric // Charset of secret.
    );
    let shares = ss.generate_shares(secret).unwrap();
    let reconstructed_secret = reconstruct_secret(&shares[0..3].to_vec(), ss).unwrap();
    assert_eq!(reconstructed_secret, secret);
}

```
