mod utils;
#[allow(dead_code)]

fn main() {
    println!("Hello World");
    println!("{:?}", utils::modinv(-18, 13));
    println!("{:?}", utils::random_polynomial(10, 50, 41));
    let coeff = utils::random_polynomial(3, 13, 103);
    println!("{:?}", coeff);
    let points = utils::get_polynomial_points(coeff.clone(), 4, 103);
    println!("{:?}", points);
    let res = utils::mod_lagrange_interpolation(points.clone(), 103);
    println!("{}", res);
    println!("{}", utils::int_to_charset(0, "0123456789abcdef".to_string()));
    println!("{}", utils::charset_to_int("abcdef123450123".to_string(), "0123456789abcdef".to_string()));
}