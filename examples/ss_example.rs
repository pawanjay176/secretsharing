extern crate secretsharing;

use secretsharing::ss::{reconstruct_secret, Charset, SecretSharing};

fn main() {
    let t = 3;
    let n = 5;
    let secret = "HelloWorld123";
    let mut ss = SecretSharing::new(t, n, Charset::Alphanumeric);
    let shares = ss.generate_shares(secret).unwrap();
    let reconstructed_secret = reconstruct_secret(&shares[0..3].to_vec(), ss).unwrap();
    assert_eq!(reconstructed_secret, secret);
}
