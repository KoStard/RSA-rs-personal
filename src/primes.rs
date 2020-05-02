use num::bigint::{RandBigInt, ToBigInt};
use num::{FromPrimitive, ToPrimitive, Integer};
use num::{BigInt, BigUint};
use rand::prelude::*;
use rand::Rng;
use std::str::FromStr;
use num::pow::pow;
use num::integer::gcd;
use crate::common::power_e_mod_n;

fn generate_big_number(digits: usize, rng: &mut ThreadRng) -> BigUint {
    let l = BigUint::from_str(
        &("1".to_owned() + &std::iter::repeat("0").take(digits - 1).collect::<String>()[..]),
    )
        .unwrap();
    let r =
        BigUint::from_str(&std::iter::repeat("9").take(digits).collect::<String>()[..]).unwrap();
    rng.gen_biguint_range(&l, &r)
}

fn pw(n: BigInt, mut e: BigInt) -> BigInt {
    if n == BigInt::from_i8(-1).unwrap() {
        return if e.is_even() {
            BigInt::from_i8(1).unwrap()
        } else {
            BigInt::from_i8(-1).unwrap()
        }
    }
    let mx = BigInt::from_i128(18_446_744_073_709_551_615).unwrap();
    let mut res = BigInt::from_u8(1).unwrap();
    while e > mx {
        res = res.clone() * pow(n.clone(), mx.to_usize().unwrap());
        e = e - mx.clone();
    }
    res.clone() * pow(n.clone(), e.to_usize().unwrap())
}

fn mno_pw(e: BigInt) -> BigInt {
    if e.is_even() {BigInt::from_i8(1).unwrap()}
    else {BigInt::from_i8(-1).unwrap()}
}

// Maybe this is wrong, because only integers are considered
fn jacobi_function(a: BigInt, b: BigInt) -> BigInt {
    // println!("J {} {}", a, b);
    let one = BigInt::from_u8(1).unwrap();
    let two = BigInt::from_u8(2).unwrap();
    let min_one = BigInt::from_i8(-1).unwrap();
    let zero = BigInt::from_u8(0).unwrap();
    if a == zero {
        return zero;
    }
    if a == one {
        one
    } else if a.clone() % 2 == zero {
        let b2 = pow(b.clone(), 2);
        jacobi_function(a.clone() / two.clone(), b.clone()) * mno_pw(((b2 - one.clone()) / BigInt::from_u8(8).unwrap()))
    } else {
        jacobi_function(b.clone() % a.clone(), a.clone())
            * mno_pw(((a.clone() - one.clone()) * (b.clone() - one.clone())
                      / BigInt::from_u8(4).unwrap()))
    }
}

pub fn check_for_a(n: &BigUint, a: BigUint) -> bool {
    // println!("{} {}", n, a);
    if n.is_even() { false } else {
        let one = BigInt::from_u8(1).unwrap();
        let two = BigInt::from_u8(2).unwrap();
        // println!("Before {} {}", a.to_bigint().unwrap(), n.to_bigint().unwrap());
        // println!("J = {}, r = {}",
        // jacobi_function(a.to_bigint().unwrap(), n.to_bigint().unwrap()).mod_floor(&n.to_bigint().unwrap()),
        //     1
        //     // power_e_mod_n(a.clone(), ((n.clone() - BigUint::from_u8(1).unwrap()) / BigUint::from_u8(2).unwrap()), n.clone())
        // );
        jacobi_function(a.to_bigint().unwrap(), n.to_bigint().unwrap()).mod_floor(&n.to_bigint().unwrap())
            == BigInt::from(power_e_mod_n(a.clone(), ((n.clone() - BigUint::from_u8(1).unwrap()) / BigUint::from_u8(2).unwrap()), n.clone()))
            && gcd(a.clone(), n.clone()) == BigUint::from_u8(1).unwrap()
    }
}

pub fn check(n: BigUint, rng: &mut ThreadRng) -> bool {
    if n.is_even() { false } else {
        for _ in 0..100 {
            if !check_for_a(&n, rng.gen_biguint_range(&BigUint::from_u8(2).unwrap(), &n)) {
                return false;
            }
        }
        return true;
    }
}

pub fn generate_prime_number(digits: usize, rng: &mut ThreadRng) -> BigUint {
    let mut count = 0;
    loop {
        let res = generate_big_number(100, rng);
        println!("Checking {}: {}", count, res);
        if check(res.clone(), rng) {
            return res;
        }
        count += 1;
    }
}


#[cfg(test)]
mod tests {
    use crate::primes::{jacobi_function, check_for_a, check};
    use num::{BigInt, BigUint};
    use num::FromPrimitive;
    use rand::{ThreadRng, thread_rng};
    use std::str::FromStr;

    #[test]
    fn check_with_prime_number() {
        let mut rnd = thread_rng();
        let c = check(BigUint::from_str("4384165182867240584805930970951575013697").unwrap(), &mut rnd);
        assert!(c);
    }
}