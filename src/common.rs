use num::BigUint;
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
