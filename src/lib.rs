mod crypto;
use crate::crypto::{argon2, hkdf, sha3_256};

use argon2::Params;
use eff_wordlist;
use rand::Rng;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

#[allow(dead_code)]
fn print_hex(input: &[u8]) {
    for num in input {
        print!("{:02x}", num);
    }
    println!();
}

fn encode_diceware(service_key: [u8; 32], length: u16) -> String {
    let mut encoded_passphrase = Vec::<String>::new();
    // let mut csprng = StdRng::from_seed(service_key);
    let mut csprng = ChaCha20Rng::from_seed(service_key);
    for _ in 0..length {
        let index = csprng.gen_range(0..7775);
        let word = eff_wordlist::large::LIST[index].1.to_string();
        encoded_passphrase.push(word);
    }
    encoded_passphrase.join(" ")
}

/*
    fn encode_diceware(service_key: [u8; 32], lenght: u16) -> String {
        let mut encoded_passphrase = Vec::<String>::new();

        encoded_passphrase.join(" ")
    }
*/
pub fn generate_passphrase(
    master_passphrase: String,
    service: String,
    secret: String,
    length: u16,
) -> String {
    // In Kilobytes
    const MEM_COST: u32 = u32::pow(1024, 1); // 1 GiB
    const ITER_COST: u32 = 10;
    const THREAD_COST: u32 = 8;

    let params = Params::new(MEM_COST, ITER_COST, THREAD_COST, None).unwrap();

    let master_key = argon2(master_passphrase, params, Some(secret));
    let service_key = hkdf(master_key, sha3_256(&service));
    let derived_passphrase = encode_diceware(service_key, length);
    let service_passphrase = service + " " + &length.to_string() + " " + &derived_passphrase;
    service_passphrase
}
