use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use sodiumoxide::crypto::pwhash::argon2id13;

/// Hash a password (or any other String) using argon2id13
///
/// # Arguments
///
/// * `passwd` - The password/string to hash
///
pub fn hash(passwd: &str) -> String {
    sodiumoxide::init().unwrap();

    let pwh = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();

    std::str::from_utf8(&pwh.0).unwrap().to_string()
}

/// Verify that a passwords matches a hash
///
/// # Arguments
///
/// * `og_hash` - the hash that the passwords needs to match
/// * `passwd` - the password that needs to match
///
pub fn verify_hash(og_hash: &str, passwd: &str) -> bool {
    sodiumoxide::init().unwrap();

    let hp = argon2id13::HashedPassword::from_slice(passwd.as_bytes()).unwrap();
    argon2id13::pwhash_verify(&hp, og_hash.as_bytes())
}

/// Generate a random token (i.e. string)
pub fn gen_token() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        let pw1 = "passwd";
        let pw2 = "verySecurePassword";

        let pwh1 = hash(pw1);
        let pwh2 = hash(pw2);

        let pwh11 = hash(pw1);
        let pwh22 = hash(pw2);

        assert_ne!(pwh1, pwh11);
        assert_ne!(pwh2, pwh22);

        assert_ne!(pwh1, pwh2);
    }

    // #[test]
    // fn test_verify_hash() {
    //     let pw1 = "passwd";
    //     let pw2 = "verySecurePassword";

    //     let pwh1 = hash(pw1);
    //     let pwh2 = hash(pw2);

    //     assert_eq!(verify_hash(&pwh1, pw1), true);
    //     assert_eq!(verify_hash(&pwh2, pw2), true);

    //     assert_eq!(verify_hash(&pwh1, pw2), true);
    //     assert_eq!(verify_hash(&pwh2, pw1), true);
    // }
}
