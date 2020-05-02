use num::{BigUint, FromPrimitive};
use std::str::FromStr;

mod block_manipulations;
mod common;
mod decrypt;
mod encrypt;
mod primes;
mod keys;

use primes::check;
use rand::thread_rng;
use crate::primes::{check_for_a, generate_prime_number};
use crate::keys::Keys;
use crate::block_manipulations::{bytes_to_blocks, blocks_to_bytes};
use encrypt::encrypt;
use decrypt::decrypt;

fn main() {
    let mut rnd = thread_rng();
    // let c = check(
    //     BigUint::from_str("5202642720986189087034837832337828472969800910926501361967872059486045713145450116712488685004691421").unwrap(),
    //     &mut rnd
    // );
    // println!("{}", c);
    // println!("{}", generate_prime_number(100, &mut rnd));


    // let k = Keys::generate(&mut rnd);
    // println!("{:?}", k);
    // println!("p: {}", k.p);
    // println!("q: {}", k.q);
    // println!("n: {}", k.n);
    // println!("d: {}", k.d);
    // println!("e: {}", k.e);

    let k1 = Keys::new(
        "4654040924660024768900715833821363734994047150613870501290053347904903606056973486127500704994154901",
        "5212783793155127111640811225579291092774470066275177683018932813978821991775338476131413627068891717",
        "24260509104748479076404861893058064461820600661827956736601675753695852806526905325553096751859016242149496164619314316527697222171348515866295039499144919772545611453927096220408989825993733093855017",
        "7584805243284008508140448027443226735062708293631957918373169723708950460359401309376361003441630199",
        "7113808099192975333520781629702377109975891664359374001900007116811936458040870973011783758309687597366009390927249608565736089120827628979911084442519251747647414337101564414349950507829208896347799",
    );

    let k2 = Keys::new(
        "2118435091873799775359069479877183631640150565996625967502966675255755099103483865152456479134702667",
        "2367051566660422205957849612905576395225559706023558303933245622221623712415608324229689299724054721",
        "5014445103088293209187948286734142617403853142971717973972700106119458431750357389726981265162830841325621084875055946855745437005998970655643689263708032984169768223333266601621588621179893972640907",
        "5982030245941972213115038951604830620422622741427780195586255012209047942942765886018753462049863207",
        "1765850179410980513055802893995280889004001914524363647910539087313737720853898647147686341379298203875662145665494695687201529431725919578908471964513357127193908538035770372131270025192448457211543",
    );

    let message = "Hello world, this is message encrypted by RSA algorithm implemented by Kostard! I can add some more message here, to be sure that this will be splitted to multiple blocks, so that we can see how is the system handling these cases.";
    let blocks = bytes_to_blocks(Vec::from(message.as_bytes()), k1.n.clone());
    let signed_blocks = blocks.iter().map(|block| encrypt(
        block.clone(),
        k1.d.clone(),
        k1.n.clone(),
    )).collect();
    let signed_message_bytes = blocks_to_bytes(signed_blocks);

    let mut encrypted_blocks = bytes_to_blocks(signed_message_bytes, k2.n.clone())
        .iter()
        .map(|block| encrypt(
            block.clone(),
            k2.e.clone(),
            k2.n.clone(),
        ))
        .collect::<Vec<BigUint>>();

    println!("{:?}", encrypted_blocks);

    let decrypted_but_signed_blocks = encrypted_blocks.iter().map(|block| decrypt(
        block.clone(),
        k2.d.clone(),
        k2.n.clone()
    )).collect::<Vec<BigUint>>();

    let decrypted_but_signed_bytes = blocks_to_bytes(decrypted_but_signed_blocks.clone());

    let mut decrypted_blocks = bytes_to_blocks(decrypted_but_signed_bytes, k1.n.clone())
        .iter()
        .map(|block|decrypt(
            block.clone(),
            k1.e.clone(),
            k1.n.clone(),
        ))
        .collect::<Vec<BigUint>>();

    let decrypted_bytes = blocks_to_bytes(decrypted_blocks.clone());
    let decrypted_message = String::from_utf8(decrypted_bytes).unwrap();

    println!("{}", decrypted_message);
}
