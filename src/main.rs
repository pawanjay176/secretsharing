extern crate num;
extern crate rand;

mod ss;
mod utils;

use num::bigint::BigInt;
use ss::{reconstruct_secret, SecretSharing};

fn main() {
    println!("Hello World");
    // Usage
    let t = 3;
    let n = 5;
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let prime_str = b"10407932194664399081925240327364085538615262247266704805319112350403608059673360298012239441732324184842421613954281007791383566248323464908139906605677320762924129509389220345773183349661583550472959420547689811211693677147548478866962501384438260291732348885311160828538416585028255604666224831890918801847068222203140521026698435488732958028878050869736186900714720710555703168729087";
    let prime = BigInt::parse_bytes(prime_str, 10).unwrap();
    // println!("{}", prime);
    let secret = "heyguyzzzz";
    let ss = SecretSharing::new(t, n, charset, prime).unwrap();
    let shares = ss.generate_shares(secret).unwrap();
    let reconstructed_secret = reconstruct_secret(shares, ss).unwrap();
    println!("{}", reconstructed_secret);
    assert_eq!(reconstructed_secret, secret);

    // println!("{}", utils::egcd(&(-1).into(), &13.into()).0);
    // println!("{}", utils::egcd(&27.into(), &18.into()).0);
    // println!("{}", utils::modinv(&(-1).into(), &13.into()).unwrap());
    // let coeff = utils::random_polynomial(t - 1, 100.into(), &103.into());
    // for c in &coeff {
    //     println!("{}", c);
    // }
    // let points = utils::get_polynomial_points(coeff.clone(), n, &103.into());
    // for (p, q) in &points {
    //     println!("{} {}", p, q);
    // }

    // let v = points[0..t as usize].to_vec();
    // println!("Number of shares: {}", v.len());
    // let res = utils::mod_lagrange_interpolation(&0.into(), v, &103.into());
    // println!("{}", res.unwrap());
    // println!(
    //     "{}",
    //     utils::int_to_charset_repr(123.into(), "0123456789abcdef").unwrap()
    // );
    // println!(
    //     "{}",
    //     utils::charset_repr_to_int(
    //         "c4bbcb1fbec99d65bf59d85c8cb62ee2db963f0fe106f483d9afa73bd4e39a8a",
    //         "0123456789abcdef"
    //     )
    //     .unwrap()
    // );
    // let points = utils::secret_int_to_points(100.into(), 2, 3, &103.into());
    // for point in &points {
    //     println!("{} {}", point.0, point.1);
    // }

    // println!(
    //     "{}",
    //     utils::points_to_secret_int(points, &103.into()).unwrap()
    // );

    // let share_str =
    //     utils::point_to_share_str(&(4660.into(), 4123.into()), "0123456789abcdef").unwrap();

    // let (a, b) = utils::share_str_to_point(share_str.as_str(), "0123456789abcdef").unwrap();
    // println!("{} {}", a, b);

    // // Usage
    // let charset = "0123456789abcdef";
    // let prime = "10407932194664399081925240327364085538615262247266704805319112350403608059673360298012239441732324184842421613954281007791383566248323464908139906605677320762924129509389220345773183349661583550472959420547689811211693677147548478866962501384438260291732348885311160828538416585028255604666224831890918801847068222203140521026698435488732958028878050869736186900714720710555703168729087";
    // let ss: SecretSharing = SecretSharing::new(t, n, charset, prime);
    // let shares = ss.generate_shares(secret);
    // assert_eq!(reconstruct_secret(shares, t, n, charset, prime), secret);
}
