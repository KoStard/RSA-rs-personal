use num::{BigUint, FromPrimitive, Signed};
use crate::primes::generate_prime_number;
use rand::{ThreadRng, thread_rng};
use std::cmp::max;
use crate::common::multiplicative_inverse;
use num::bigint::{RandBigInt, ToBigInt};
use std::str::FromStr;

#[derive(Debug)]
pub struct Keys {
    pub p: BigUint,
    pub q: BigUint,
    pub n: BigUint,
    pub d: BigUint,
    pub e: BigUint,
}


impl Keys {
    pub fn generate(rng: &mut ThreadRng) -> Self {
        loop {
            let k1 = generate_prime_number(100, rng);
            let mut k2 = generate_prime_number(100, rng);
            while k2 == k1 { k2 = generate_prime_number(100, rng); }

            let mut k3 = generate_prime_number(100, rng);
            while k3 == k1 || k3 == k2 { k3 = generate_prime_number(100, rng); }

            let d = max(k1.clone(), max(k2.clone(), k3.clone()));
            let p = if k1 == d { k2.clone() } else { k1.clone() };
            let q = if k1 != d && k1 != p { k1 } else { if k2 != d && k2 != p { k2 } else { k3 } };
            let fi = (p.clone() - BigUint::from_u8(1).unwrap()) * (q.clone() - BigUint::from_u8(1).unwrap());
            let n = p.clone() * q.clone();
            let e_o = multiplicative_inverse(
                fi.to_bigint().unwrap(),
                d.to_bigint().unwrap()
            );
            if e_o.is_some() {
                let e = e_o.unwrap();
                if e.is_negative() {continue;}
                let e = e.to_biguint().unwrap();
                if e.to_string().len() < 100 { continue; }
                return Keys {
                    p,
                    q,
                    n,
                    d,
                    e,
                };
            } else {
                println!("Failed, trying again");
            }
        }
    }

    pub fn new(p: &str,
               q: &str,
               n: &str,
               d: &str,
               e: &str) -> Self {
        Keys {
            p: BigUint::from_str(p).unwrap(),
            q: BigUint::from_str(q).unwrap(),
            n: BigUint::from_str(n).unwrap(),
            d: BigUint::from_str(d).unwrap(),
            e: BigUint::from_str(e).unwrap(),
        }
    }
}
