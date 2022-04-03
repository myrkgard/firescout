//! Convienence functions for generation of random numbers and bytes.

use num_bigint::BigUint;
use num_traits::Zero;
use rand::Rng;
use rand::RngCore;

/// Random bytes.
pub fn get_random_bytes(count: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; count];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

/// Random BigUint between 0, inclusive, and max, inclusive.
/// Throws if max is negative.
pub fn get_random_biguint(max: BigUint) -> BigUint {
    get_random_biguint_from_range(BigUint::zero(), max)
}

/// Random BigUint between lower, inclusive, and upper, inclusive.
/// Throws if lower is negative, throws if upper is negative, throws
/// if upper is not greater than lower.
pub fn get_random_biguint_from_range(lower: BigUint, upper: BigUint) -> BigUint {
    rand::thread_rng().gen_range(lower..=upper)
}
