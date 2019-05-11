extern crate secretsharing;

use secretsharing::ss::{reconstruct_secret, Charset, SecretSharing};

fn main() {
    let t = 3;
    let n = 5;
    let secret = "HelloWorld123";
    let mut ss = SecretSharing::new(t, n, Charset::Alphanumeric).unwrap();
    let shares = ss.generate_shares(secret).unwrap();
    let reconstructed_secret = reconstruct_secret(&shares, ss).unwrap();
    println!("{}", reconstructed_secret);
    assert_eq!(reconstructed_secret, secret);
}
