use crate::common::power_e_mod_n;
use num::bigint::BigUint;
use num::FromPrimitive;
use std::ops::{BitAnd, Mul};

pub fn encrypt(M: BigUint, e: BigUint, n: BigUint) -> BigUint {
    power_e_mod_n(M, e, n)
}

#[cfg(test)]
mod tests {
    use crate::encrypt::encrypt;
    use num::BigUint;
    use num::FromPrimitive;

    #[test]
    fn it_works() {
        let result = encrypt(
            BigUint::from_i64(920).unwrap(),
            BigUint::from_i64(17).unwrap(),
            BigUint::from_i64(2773).unwrap(),
        );
        assert_eq!(result, BigUint::from_i64(948).unwrap());
    }
}
