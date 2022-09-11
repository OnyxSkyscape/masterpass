# Master Passphrase Key Derivation Algorithm

## Definitions
 - SPwd: Service Passphrase
 - MPwd: Master Passphrase
 - Info: Service specific information (like name or other identifier). This should be public information, and will be used to remind the user of the password.
 - Pepper: additional secret salt (pepper), used as salt for Argon2 (optional and unique for every SPwd, only used for Mpwd to prevent leakage of SPwd in case of Mpwd getting compromised)
 - Length: desired length of SPwd, in words

 - SKey: High entropy service key

 - Argon2d: the Argon2d key derivation function for low entropy input
 - HKDF_extract: HMAC based 
 - HKDF_expand: HMAC based key derivation function for high entropy input (only the extract phrase is used for sufficiently)
 - SHA3-256: SHA3-256 cryptographic hash function used for 
 - EncodeDice: EFF diceware wordlist encoding function

 - ChaCha20Rng: CSPRNG ChaCha20 block cipher seeded with SKey

 - mem_cost: memory cost for Argon2
 - iter_cost: number of iteration for Argon2
 - thread_cost: number of threads for Argon2

## Design goals
 - One master passphrase for all services
 - **TODO**


## Algorithm
EFF diceware encoding algorithm used in demo 
```
// assume the EFF long wordlist is in an array indexed from 0 to 7775 (inclusive)
// 

fn EncodeDice(SKey: [u8; 32], lenght: u16) -> String {
    let mut encoded_passphrase = Vec::<String>::new();
    let mut csprng = ChaCha20Rng::from_seed(SKey);
    for _ in 0..length {
        let index = csprng.gen_range(0..7775);
        let word = eff_wordlist::large::LIST[index].1.to_string();
        encoded_passphrase.push(word);
    }
    encoded_passphrase.join(" ")
}
```
### v1 (for reference only; to be removed from the official design document)
<!-- Why? --> 
```
Metadata = Info + " " + length <!-- For better memorization purposes-->
MKey = Argon2d(pwd: MPwd, salt: SHA3(Pepper), mem_cost) <!-- memory-hard, resistant against time-space tradeoff attacks -->
SKey = HKDF_expand(digest: SHA3-256, prk: MKey, info: HKDF_extract(SHA3_256(Info)))
SPwd = Metadata + " " + EncodeDice(SKey, length)
```
### v2
```
Metadata = Info + " " + length
MKey = Argon2d(pwd: MPwd, salt: SHA3(Pepper), parameters)
// TODO
```