extern crate num;
extern crate rand;

use num::bigint::{BigInt, RandBigInt};
use num::{One, Zero};
use num::traits::ToPrimitive;

#[derive(Debug)]
pub enum SSError {
    NoModInverse,
    InvalidCharacter,
}

pub fn egcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b.clone(), Zero::zero(), One::one())
    } else {
        let (gcd, x, y) = egcd(&(b % a), a);
        (gcd, y - (b / a) * &x, x)
    }
}

pub fn modinv(a: &BigInt, prime: &BigInt) -> Result<BigInt, SSError> {
    let (gcd, x, _) = egcd(a, prime);
    if !gcd.is_one() {
        Err(SSError::NoModInverse)
    } else {
        Ok(((x % prime) + prime) % prime)
    }
}

pub fn random_polynomial(degree: u64, intercept: BigInt, upper_bound: &BigInt) -> Vec<BigInt> {
    let mut coeff = vec![intercept];
    let mut rng = rand::thread_rng();
    for _ in 0..degree {
        let c = rng.gen_bigint_range(&0.into(), upper_bound);
        coeff.push(c);
    }
    coeff
}

pub fn get_polynomial_points(
    coeff: Vec<BigInt>,
    num_points: u64,
    prime: &BigInt,
) -> Vec<(BigInt, BigInt)> {
    let mut points: Vec<(BigInt, BigInt)> = Vec::new();
    for i in 1..=num_points {
        let mut y = coeff[0].clone();
        for j in 1..coeff.len() {
            let exp: BigInt = (i.pow(j as u32)) % prime;
            let term = (&coeff[j] * exp) % prime;
            y = (y + term) % prime;
        }
        points.push((i.into(), y));
    }
    points
}

pub fn mod_lagrange_interpolation(points: Vec<(BigInt, BigInt)>, prime: &BigInt) -> Result<BigInt, SSError> {
    let mut res: BigInt = Zero::zero();
    let n = points.len();
    let x_values: Vec<BigInt> = points.clone().into_iter().map(|(x, _)| x).collect();
    let y_values: Vec<BigInt> = points.into_iter().map(|(_, y)| y).collect();
    for i in 0..n {
        let mut num: BigInt = One::one();
        let mut den: BigInt = One::one();
        for j in 0..n {
            if i == j {
                continue;
            } else {
                num = (num * -&x_values[j]) % prime;
                den = (den * (&x_values[i] - &x_values[j])) % prime;
            }
        }
        let lagrange_polynomial = num * modinv(&den, prime)?;
        res = (res + prime + (&y_values[i] * lagrange_polynomial)) % prime;
    }
    Ok((res + prime) % prime)
}

pub fn int_to_charset(mut value: BigInt, charset: &str) -> Result<String, SSError> {
    if value == Zero::zero() {
        match charset.chars().nth(0) {
            None => Err(SSError::InvalidCharacter),
            Some(x) => Ok(x.to_string())
        }
    } else {
        let mut res = String::new();
        while value > Zero::zero() {
            let digit: usize = (&value % charset.len()).to_usize().unwrap();
            value = &value / charset.len();
            res.push(charset.chars().nth(digit).ok_or(SSError::InvalidCharacter)?);
        }
        Ok(res.chars().rev().collect::<String>())
    }
}

pub fn charset_to_int(val: &str, charset: &str) -> Result<BigInt, SSError> {
    let mut res: BigInt = Zero::zero();
    for c in val.chars() {
        res = res * charset.len() + charset.find(c).ok_or(SSError::InvalidCharacter)?;
    }
    Ok(res)
}

// pub fn secret_int_to_points(
//     secret_int: i64,
//     t: i64,
//     n: i64,
//     prime: i64,
// ) -> Result<Vec<(i64, i64)>, String> {
//     if t < 2 {
//         Err("Threshold should be >=2".to_string())
//     } else {
//         if t > n {
//             Err("t cannot be greater than n".to_string())
//         } else {
//             let coeff = random_polynomial(t - 1, secret_int, prime);
//             println!("{:?}", coeff);
//             Ok(get_polynomial_points(coeff, n, prime))
//         }
//     }
// }

// pub fn points_to_secret_int(points: Vec<(i64, i64)>, prime: i64) -> i64 {
//     mod_lagrange_interpolation(points, prime)
// }
