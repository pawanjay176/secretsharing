extern crate num;
extern crate rand;

mod ss;
mod utils;

use ss::{reconstruct_secret, Charset, SecretSharing};

fn main() {
    println!("Hello World");
    // Usage
    let t = 3;
    let n = 5;
    let secret =
        "oasjdajskdjkasnkdasnkjdnkjasndkjsankjdnkjasndkjnakjdnkjankjdnkjasndjkankjsdnakjdnkas";
    let mut ss = SecretSharing::new(t, n, Charset::Alphanumeric).unwrap();
    let shares = ss.generate_shares(secret).unwrap();
    let reconstructed_secret = reconstruct_secret(&shares, ss).unwrap();
    println!("{}", reconstructed_secret);
    assert_eq!(reconstructed_secret, secret);
}
