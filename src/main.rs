mod utils;
extern crate rand;
// extern crate num;

fn main() {
    println!("Hello World");

    println!("{:?}", utils::egcd(&18.into(), &27.into()));
    println!("{:?}", utils::modinv(&(-18).into(), &13.into()));
    let coeff = utils::random_polynomial(3, 13.into(), &103.into());
    for p in &coeff {
        println!("{}", p);
    }
    let points = utils::get_polynomial_points(coeff.clone(), 4, &103.into());
    for (p, q) in &points {
        println!("{} {}", p, q);
    }

    let res = utils::mod_lagrange_interpolation(points.clone(), &103.into());
    println!("{}", res.unwrap());
    println!(
        "{}",
        utils::int_to_charset(123.into(), "0123456789abcdef").unwrap()
    );
    println!(
        "{}",
        utils::charset_to_int(
            "c4bbcb1fbec99d65bf59d85c8cb62ee2db963f0fe106f483d9afa73bd4e39a8a",
            "0123456789abcdef"
        ).unwrap()
    );
    // let points = utils::secret_int_to_points(100, 2, 3, 103);
    // println!("{:?}", points);
    // println!("{}", utils::points_to_secret_int(points.unwrap(), 103));

    // // Usage
    // let ss: SecretSharing = SecretSharing::new(t, n, charset, prime);
    // let shares = ss.generate_shares(secret);
    // assert_eq!(reconstruct_secret(shares, t, n, charset, prime), secret);
}
