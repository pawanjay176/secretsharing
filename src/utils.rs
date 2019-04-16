extern crate rand;

use rand::Rng;

pub fn egcd(a: i64,b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    }
    else {
        let (gcd, x, y) = egcd(b % a, a);
        (gcd, y - (b/a) * x, x)
    }
}

pub fn modinv(a: i64, prime: i64) -> Option<i64> {
    let (gcd, x, _) = egcd(a, prime);
    
    if gcd != 1 {
        None
    }
    else {
        Some(((x % prime) + prime) % prime)
    }
}

pub fn random_polynomial(degree: i64, intercept: i64, upper_bound: i64) -> Vec<i64> {
    let mut coeff = vec![intercept];
    let mut rng = rand::thread_rng();
    for _ in 0..degree {
        let c = rng.gen_range(0, upper_bound);
        coeff.push(c);
    }
    coeff
}


pub fn get_polynomial_points(coeff: Vec<i64>, num_points: i64, prime: i64) -> Vec<(i64, i64)> {
    let mut points: Vec<(i64, i64)> = Vec::new();
    for i in 1..=num_points {
        let mut y = coeff[0];
        for j in 1..coeff.len() {
            let exp: i64 = (i.pow(j as u32)) % prime;
            let term = (coeff[j] * exp) % prime;
            y = (y + term) % prime;
        }
        points.push((i,y));
    }
    points
}

pub fn mod_lagrange_interpolation(points: Vec<(i64, i64)>, prime: i64) -> i64 {
    let mut res: i64 = 0;
    let x_values: Vec<i64> = points.iter().map(|(x, _)| *x).collect();    
    let y_values: Vec<i64> = points.iter().map(|(_, y)| *y).collect();
    for i in 0..points.len(){
        let mut num: i64 = 1;
        let mut den: i64 = 1;
        for j in 0..points.len(){
            if i == j {
                continue
            }
            else {
                num = (num * -x_values[j]) % prime;
                den = (den * (x_values[i] - x_values[j])) % prime;
            }
        }
        let lagrange_polynomial = num * modinv(den, prime).unwrap();
        res = (res + prime + (y_values[i] * lagrange_polynomial)) % prime;
    }
    res
}

pub fn int_to_charset(val: u64, charset: String) -> String {
    if val == 0 {
        charset.chars().nth(0).unwrap().to_string()
    }
    else {
        let mut res = String::new();
        let mut value = val.clone();
        while value > 0 {
            value = value / charset.len() as u64;
            let digit = value % charset.len() as u64;
            res.push(charset.chars().nth(digit as usize).unwrap());
        }
        res
    }
}

pub fn charset_to_int(val: String, charset: String) -> u64 {
    let mut res: u64 = 0;
    for c in val.chars() {
        res = res * charset.len() as u64 + charset.find(c).unwrap() as u64;
    }
    res
}

