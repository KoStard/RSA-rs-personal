use num::{BigUint, FromPrimitive};
use std::str::FromStr;

mod block_manipulations;
mod common;
mod decrypt;
mod encrypt;
mod primes;

use primes::check;
use rand::thread_rng;
use crate::primes::{check_for_a, generate_prime_number};

fn main() {
    let mut rnd = thread_rng();
    // let c = check(
    //     BigUint::from_str("5202642720986189087034837832337828472969800910926501361967872059486045713145450116712488685004691421").unwrap(),
    //     &mut rnd
    // );
    // println!("{}", c);
    println!("{}", generate_prime_number(100, &mut rnd));
}
