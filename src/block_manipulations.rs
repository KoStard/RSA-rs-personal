use num::FromPrimitive;
use num::{BigUint, ToPrimitive};

pub fn bytes_to_blocks(bytes: Vec<u8>, n: BigUint) -> Vec<BigUint> {
    let initial = BigUint::from_u8(0).unwrap();
    let mut current = initial.clone();
    let mut blocks: Vec<BigUint> = Vec::new();
    for byte in bytes {
        let c2 = (&current << 8) + byte;
        if c2 < n {
            current = c2;
        } else {
            blocks.push(current.clone());
            current = BigUint::from_u8(byte).unwrap();
        }
    }
    if current != initial {
        blocks.push(current);
    }
    blocks
}

pub fn blocks_to_bytes(blocks: Vec<BigUint>) -> Vec<u8> {
    let mut bytes = Vec::new();
    let zero = BigUint::from_u8(0).unwrap();
    for mut block in blocks {
        let mut block_bytes = Vec::new();
        while block > zero {
            let byte: BigUint = block.clone() % BigUint::from_u16(256).unwrap();
            block = block >> 8;
            block_bytes.push(byte.to_u8().unwrap());
        }
        block_bytes.reverse();
        bytes.append(&mut block_bytes);
    }
    bytes
}

#[cfg(test)]
mod tests {
    use crate::block_manipulations::{blocks_to_bytes, bytes_to_blocks};
    use num::BigUint;
    use num::FromPrimitive;

    #[test]
    fn it_works() {
        let result = bytes_to_blocks(
            String::from("Hello world").into_bytes(),
            BigUint::from_u64(277300).unwrap(),
        );
        println!("{:?}", String::from_utf8(blocks_to_bytes(result)));
    }
}
