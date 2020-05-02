use crate::common::power_e_mod_n;
use num::bigint::BigUint;
use num::FromPrimitive;
use std::ops::{BitAnd, Mul};

pub fn decrypt(C: BigUint, d: BigUint, n: BigUint) -> BigUint {
    power_e_mod_n(C, d, n)
}

#[cfg(test)]
mod tests {
    use crate::decrypt::decrypt;
    use num::BigUint;
    use num::FromPrimitive;

    #[test]
    fn it_works() {
        let result = decrypt(
            BigUint::from_i64(948).unwrap(),
            BigUint::from_i64(157).unwrap(),
            BigUint::from_i64(2773).unwrap(),
        );
        assert_eq!(result, BigUint::from_i64(920).unwrap());
    }
}
