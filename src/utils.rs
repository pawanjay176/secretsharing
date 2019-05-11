#![allow(clippy::needless_range_loop)]
use num::bigint::{BigInt, RandBigInt};
use num::traits::ToPrimitive;
use num::{One, Zero};

#[derive(Debug)]
pub enum SSError {
    /// Modular inverse does not exist.
    NoModInverse,
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
    /// Insufficient shares to reconstruct secret.
    InsufficientShares,
    /// Field prime not set.
    PrimeNotSet,
    /// Secret too big. We don't have a prime greater than integer representation of secret.
    SecretTooLarge,
}

/// Point on the polynomial with coefficients in F_{p}
#[derive(Debug, Clone)]
pub struct Point(BigInt, BigInt);

/// Function which returns vector of prime numbers
fn primes() -> Vec<BigInt> {
    vec![3.into(), 7.into(), 31.into(), 127.into(), 8191.into(), 131_071.into(), 524_287.into(), 2_147_483_647.into(), BigInt::parse_bytes(b"2305843009213693951", 10).unwrap(), BigInt::parse_bytes(b"618970019642690137449562111", 10).unwrap(), BigInt::parse_bytes(b"162259276829213363391578010288127", 10).unwrap(), BigInt::parse_bytes(b"170141183460469231731687303715884105727", 10).unwrap(), BigInt::parse_bytes(b"115792089237316195423570985008687907853269984665640564039457584007913129640233", 10).unwrap(), BigInt::parse_bytes(b"2135987035920910082395021706169552114602704522356652769947041607822219725780640550022962086936603", 10).unwrap(),BigInt::parse_bytes(b"39402006196394479212279040100143613805079739270465446667948293404245721771497210611414266254884915640806627990307047", 10).unwrap(), BigInt::parse_bytes(b"6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151", 10).unwrap(), BigInt::parse_bytes(b"531137992816767098689588206552468627329593117727031923199444138200403559860852242739162502265229285668889329486246501015346579337652707239409519978766587351943831270835393219031728127", 10).unwrap(), BigInt::parse_bytes(b"10407932194664399081925240327364085538615262247266704805319112350403608059673360298012239441732324184842421613954281007791383566248323464908139906605677320762924129509389220345773183349661583550472959420547689811211693677147548478866962501384438260291732348885311160828538416585028255604666224831890918801847068222203140521026698435488732958028878050869736186900714720710555703168729087", 10).unwrap()]
}

/// Return next prime greater than `val` from the primes list.
pub fn next_prime(val: &BigInt) -> Result<BigInt, SSError> {
    primes()
        .into_iter()
        .skip_while(|x| x <= val)
        .take(1)
        .collect::<Vec<_>>()
        .pop()
        .ok_or(SSError::SecretTooLarge)
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
        Err(SSError::NoModInverse)
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
    points: Vec<Point>,
    prime: &BigInt,
) -> Result<BigInt, SSError> {
    let mut res: BigInt = Zero::zero();
    let n = points.len();
    let x_values: Vec<BigInt> = points.clone().into_iter().map(|Point(x, _)| x).collect();
    let y_values: Vec<BigInt> = points.into_iter().map(|Point(_, y)| y).collect();
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
pub fn points_to_secret_int(points: Vec<Point>, prime: &BigInt) -> Result<BigInt, SSError> {
    mod_lagrange_interpolation(&0.into(), points, prime)
}

/// Converts point representation of share to a string representation
/// of the form "x-y" where x and y are the string representation of
/// the given point in the charset.
pub fn point_to_share_str(point: &(BigInt, BigInt), charset: &str) -> Result<String, SSError> {
    if charset.contains('-') {
        return Err(SSError::InvalidCharset);
    }
    let (x, y) = point;
    let mut x_str = int_to_charset_repr(x.clone(), charset)?;
    let y_str = int_to_charset_repr(y.clone(), charset)?;

    x_str.push_str("-");
    x_str.push_str(&y_str);
    Ok(x_str)
}

/// Convert string representation of point back to original point.
pub fn share_str_to_point(share: &str, charset: &str) -> Result<Point, SSError> {
    if charset.contains('-') {
        return Err(SSError::InvalidCharset);
    }
    let (x_str, y_str) = match *share.split('-').collect::<Vec<_>>().as_slice() {
        [x, y] => (x, y),
        _ => return Err(SSError::InvalidShare),
    };
    Ok(Point(
        charset_repr_to_int(x_str, charset)?,
        charset_repr_to_int(y_str, charset)?,
    ))
}
