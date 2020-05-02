use num::{BigUint, Integer, Zero, BigInt, One};
use num::FromPrimitive;
use std::ops::{BitAnd, Mul};

pub fn get_bytes(mut e: BigUint) -> Vec<bool> {
    let mut ei: Vec<bool> = Vec::new();
    let zero = BigUint::from_i64(0).unwrap();
    let one = BigUint::from_i64(1).unwrap();

    while e > zero {
        let k = e.clone().bitand(&one).eq(&one);
        ei.push(k);
        e = e >> 1;
    }

    ei
}

pub fn power_e_mod_n(m: BigUint, e: BigUint, n: BigUint) -> BigUint {
    let ei: Vec<bool> = get_bytes(e);

    let mut p = BigUint::from_i64(1).unwrap();

    ei.iter().rev().for_each(|el| {
        p = (p.clone().mul(p.clone())) % &n; // not safe for the boundaries
        if *el {
            p = (p.clone().mul(m.clone())) % &n;
        }
    });
    p
}

// gcd -> multiplicative inverse
pub fn multiplicative_inverse(mut x0: BigInt, mut x1: BigInt) -> Option<BigInt> {
    let mut a0 = BigInt::from_u8(1).unwrap();
    let mut b0 = BigInt::from_u8(0).unwrap();
    let mut a1 = BigInt::from_u8(0).unwrap();
    let mut b1 = BigInt::from_u8(1).unwrap();
    loop {
        let xi = x0.mod_floor(&x1);
        if xi.is_zero() {
            return if !x1.is_one() {
                None
            } else {
                Some(b1)
            };
        }
        let ai = a0.clone() - a1.clone() * (x0.clone() / x1.clone());
        let bi = b0.clone() - b1.clone() * (x0.clone() / x1.clone());
        a0 = a1;
        a1 = ai;
        b0 = b1;
        b1 = bi;
        x0 = x1;
        x1 = xi;
    }
}


#[cfg(test)]
mod tests {
    use crate::common::multiplicative_inverse;
    use num::{BigInt, FromPrimitive};

    #[test]
    fn multiplicative_inverse_test() {
        assert_eq!(multiplicative_inverse(
            BigInt::from_u128(2668).unwrap(),
            BigInt::from_u128(157).unwrap()
        ), Some(BigInt::from_u128(17).unwrap()));
    }
}
