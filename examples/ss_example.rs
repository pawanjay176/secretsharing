extern crate secretsharing;

use secretsharing::ss::{Charset, SecretSharing};

fn main() {
    let threshold = 3;
    let total_shares = 5;
    let secret = "HelloWorld123";
    let mut ss = SecretSharing::new(
        threshold,             // Minimum shares required for reconstruction.
        total_shares,          // Total shares.
        Charset::Alphanumeric, // Charset of secret.
    );
    let shares = ss.generate_shares(secret).unwrap();
    let reconstructed_secret = ss.reconstruct_secret(&shares[0..3].to_vec()).unwrap();
    assert_eq!(reconstructed_secret, secret);
}
