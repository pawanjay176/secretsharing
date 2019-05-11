extern crate num;
extern crate rand;

use num::bigint::{BigInt, RandBigInt};
use num::traits::ToPrimitive;
use num::{One, Zero};

#[derive(Debug)]
pub enum SSError {
    /// Modular inverse does not exist.
    NoModInverse(BigInt, BigInt),
    /// Character not present in charset of SecretSharing instance.
    InvalidCharacter,
    /// Threshold for secret sharing less than 2.
    LowThreshold,
    /// Threshold greated than total.
    HighThreshold,
    /// Charset cannot contain hyphen.
    InvalidCharset,
    /// Share string does not contain hyphen.
    InvalidShare,
}

/// Extended euclidean algorithm.
pub fn egcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b.clone(), Zero::zero(), One::one())
    } else {
        let (gcd, x, y) = egcd(&(b % a), a);
        (gcd, y - (b / a) * &x, x)
    }
}

/// Modular inverse of `a` w.r.t prime `p`.
pub fn modinv(a: &BigInt, prime: &BigInt) -> Result<BigInt, SSError> {
    let val = ((a % prime) + prime) % prime;
    let (gcd, x, _) = egcd(&val, prime);
    if !gcd.is_one() {
        Err(SSError::NoModInverse(a.clone(), prime.clone()))
    } else {
        Ok(((x % prime) + prime) % prime)
    }
}

/// Generates coefficients for a random polynomial.
pub fn random_polynomial(degree: u32, intercept: BigInt, upper_bound: &BigInt) -> Vec<BigInt> {
    let mut coeff = vec![intercept];
    let mut rng = rand::thread_rng();
    for _ in 0..degree {
        let c = rng.gen_bigint_range(&0.into(), upper_bound);
        coeff.push(c);
    }
    coeff
}

/// Calculates (x, y) for given polynomial.
pub fn get_polynomial_points(
    coeff: Vec<BigInt>,
    num_points: u32,
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

/// Modular lagrangian interpolation algorithm.
pub fn mod_lagrange_interpolation(
    x: &BigInt,
    points: Vec<(BigInt, BigInt)>,
    prime: &BigInt,
) -> Result<BigInt, SSError> {
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
                num = (num * (x - &x_values[j])) % prime;
                den = (den * (&x_values[i] - &x_values[j])) % prime;
            }
        }
        let lagrange_polynomial = num * modinv(&den, prime)?;
        res = (res + prime + (&y_values[i] * lagrange_polynomial)) % prime;
    }
    Ok((res + prime) % prime)
}

/// Coverts given integer to its representation in given charset.
pub fn int_to_charset_repr(mut value: BigInt, charset: &str) -> Result<String, SSError> {
    if value == Zero::zero() {
        match charset.chars().nth(0) {
            None => Err(SSError::InvalidCharacter),
            Some(x) => Ok(x.to_string()),
        }
    } else {
        let mut res = String::new();
        while value > Zero::zero() {
            let digit: usize = (&value % charset.len()).to_usize().unwrap();
            value = &value / charset.len();
            res.push(
                charset
                    .chars()
                    .nth(digit)
                    .ok_or(SSError::InvalidCharacter)?,
            );
        }
        Ok(res.chars().rev().collect::<String>())
    }
}

/// Coverts a charset representation to its corresponding integer value.
pub fn charset_repr_to_int(val: &str, charset: &str) -> Result<BigInt, SSError> {
    let mut res: BigInt = Zero::zero();
    for c in val.chars() {
        res = res * charset.len() + charset.find(c).ok_or(SSError::InvalidCharacter)?;
    }
    Ok(res)
}

/// Splits a secret to share points.
pub fn secret_int_to_points(
    secret_int: BigInt,
    t: u32,
    n: u32,
    prime: &BigInt,
) -> Vec<(BigInt, BigInt)> {
    let coeff = random_polynomial(t - 1, secret_int, prime);
    get_polynomial_points(coeff, n, prime)
}

/// Recovers secret integer from point shares.
pub fn points_to_secret_int(
    points: Vec<(BigInt, BigInt)>,
    prime: &BigInt,
) -> Result<BigInt, SSError> {
    mod_lagrange_interpolation(&0.into(), points, prime)
}

/// Converts point representation of share to a string representation
/// of the form "x-y" where x and y are the string representation of
/// the given point in the charset.
pub fn point_to_share_str(point: (BigInt, BigInt), charset: &str) -> Result<String, SSError> {
    if charset.contains("-") {
        return Err(SSError::InvalidCharset);
    }
    let (x, y) = point;
    let mut x_str = int_to_charset_repr(x, charset)?;
    let y_str = int_to_charset_repr(y, charset)?;

    x_str.push_str("-");
    x_str.push_str(&y_str);
    Ok(x_str)
}

/// Convert string representation of point back to original point.
pub fn share_str_to_point(share: &str, charset: &str) -> Result<(BigInt, BigInt), SSError> {
    if charset.contains("-") {
        return Err(SSError::InvalidCharset);
    }
    let (x_str, y_str) = match share.split('-').collect::<Vec<_>>().as_slice() {
        &[x, y] => (x, y),
        _ => return Err(SSError::InvalidShare),
    };
    Ok((
        charset_repr_to_int(x_str, charset)?,
        charset_repr_to_int(y_str, charset)?,
    ))
}
