use crate::utils;
use crate::utils::SSError;
use num::BigInt;

/// Secret sharing struct.
pub struct SecretSharing {
    /// Minimum number of shares required to recover the secret.
    threshold: u32,
    /// Total number of parties the secret is split amongst.
    total: u32,
    /// The character set used by the secret.
    charset: Charset,
    /// Prime number used for the prime field for polynomial operations.
    prime: Option<BigInt>,
}

impl SecretSharing {
    pub fn new(threshold: u32, total: u32, charset: Charset) -> Self {
        SecretSharing {
            threshold,
            total,
            charset,
            prime: None,
        }
    }

    /// Minimum number of shares required to reconstruct secret.
    #[inline]
    pub fn threshold(&self) -> u32 {
        self.threshold
    }

    /// Total number of parties the secret is split amongst.
    #[inline]
    pub fn total(&self) -> u32 {
        self.total
    }

    /// Charset in which the secret is represented.
    #[inline]
    pub fn charset(&self) -> &str {
        self.charset.charset_str()
    }

    /// Set prime in which field operations are performed.
    #[inline]
    fn set_prime(&mut self, prime: BigInt) {
        self.prime = Some(prime)
    }

    #[inline]
    /// Prime number on which the field operations are performed.
    pub fn prime(&self) -> Result<&BigInt, SSError> {
        self.prime.as_ref().ok_or(SSError::PrimeNotSet)
    }

    /// Split a secret to shares based on SecretSharing params.
    pub fn generate_shares(&mut self, secret: &str) -> Result<Vec<String>, SSError> {
        // Need threshold to be atleast 2.
        if self.threshold() < 2 {
            return Err(SSError::LowThreshold);
        }
        // Threshold can't be greater than total.
        if self.threshold() > self.total() {
            return Err(SSError::HighThreshold);
        }
        // Convert secret to its integer representation in charset.
        let secret_int = utils::charset_repr_to_int(secret, self.charset())?;
        // Set prime to be prime number greater than `secret_int`.
        self.set_prime(utils::next_prime(&secret_int)?);
        // Get point representation of shares.
        let points =
            utils::secret_int_to_points(secret_int, self.threshold(), self.total(), self.prime()?);
        // Convert point representation to string representation.
        let shares: Result<Vec<_>, SSError> = points
            .iter()
            .map(|point| utils::point_to_share_str(point, self.charset()))
            .collect();
        shares
    }
}

/// Reconstruct secret from shares.
pub fn reconstruct_secret(shares: &[String], ss: SecretSharing) -> Result<String, SSError> {
    // Not enough shares to reconstruct secret.
    if (shares.len() as u32) < ss.threshold() {
        return Err(SSError::InsufficientShares);
    }
    // Convert shares to their point representations.
    let point_shares: Result<Vec<_>, SSError> = shares
        .iter()
        .map(|share| utils::share_str_to_point(share.as_str(), ss.charset()))
        .collect();
    // Recover secret_int.
    let secret_int = utils::points_to_secret_int(point_shares?, ss.prime()?)?;
    // Convert secret_int to secret based on charset.
    utils::int_to_charset_repr(secret_int, ss.charset())
}

/// Possible charsets for secret.
pub enum Charset {
    Hex,
    Alphanumeric,
}

impl Charset {
    /// str of all possible characters in charset.
    pub fn charset_str(&self) -> &str {
        match self {
            Charset::Hex => "0123456789abcdef",
            Charset::Alphanumeric => {
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            }
        }
    }
}
