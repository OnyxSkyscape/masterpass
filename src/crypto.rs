use crypto::digest::Digest;
use crypto::hkdf;
use argon2::{Argon2, Params, Version};
use crypto::sha3::Sha3;


pub fn sha3_256(input: &str) -> [u8; 32] {
    let out = &mut [0u8; 32];
    let mut sha3_256 = Sha3::sha3_256();
    sha3_256.input_str(&input);
    sha3_256.result(out);
    *out
}

pub fn argon2(input: String, params: Params, pepper: Option<String>) -> [u8; 32] {
    let pepper = match pepper {
        Some(s) => sha3_256(&s),
        None => sha3_256(&input),
    };

    let argon2 = Argon2::new(argon2::Algorithm::Argon2d, Version::default(), params);

    let mut buf = [0u8; 32];
    argon2
        .hash_password_into(input.as_bytes(), &pepper, &mut buf)
        .unwrap();

    // println!("{:?}", buf);
    // print_hex(&buf);
    buf
}

pub fn hkdf(master_key: [u8; 32], service_info: [u8; 32]) -> [u8; 32] {
    let digest: Sha3 = Sha3::sha3_256();
    let mut out = [0u8; 32];
    let mut info_key_buf = [0u8; 32];
    hkdf::hkdf_extract(digest, b"\x00", &service_info, &mut info_key_buf);
    hkdf::hkdf_expand(digest, &master_key, &service_info, &mut out);
    // print_hex(&out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Instant;

    #[test]
    fn test_argon2() {
        let params = Params::new(1024, 1, 1, None).unwrap();
        let input = "test".to_string();
        let secret = Some("secret".to_string());
        let result = argon2(input, params, secret);
        let expected_result = [
            107, 251, 189, 209, 62, 68, 205, 174, 115, 80, 105, 90, 157, 184, 15, 218, 94, 154,
            202, 210, 121, 233, 60, 3, 134, 42, 51, 10, 211, 171, 161, 50,
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn bench_argon2() {
        const MEM_COST: u32 = 8 * 1024; // 1 MB
        const ITER_COST: u32 = 16;
        const THREAD_COST: u32 = 16;
        let params = Params::new(MEM_COST, ITER_COST, THREAD_COST, None).unwrap();
        let input = "test".to_string();
        let secret = Some("secret".to_string());

        let start = Instant::now();
        let _ = argon2(input.clone(), params.clone(), secret.clone());
        let duration = start.elapsed();
        println!(
            "\n\nArgon2d with input \"{}\", and pepper \"{}\"\n\
         and parameters {} memory cost, {} iterations, {} threads,\n\
         took {:?}\n\n",
            input,
            secret.unwrap_or_default(),
            MEM_COST,
            ITER_COST,
            THREAD_COST,
            duration
        );
    }
}
