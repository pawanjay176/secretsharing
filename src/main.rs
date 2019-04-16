mod utils;


fn main() {
    println!("Hello, world!");
    println!("{:?}", utils::modinv(-18, 13));
    println!("{:?}", utils::random_polynomial(10, 50, 41));
    println!("{:?}", utils::mod_lagrange_interpolation(vec![(1,2), (3,4), (5,6)], 100));
}
