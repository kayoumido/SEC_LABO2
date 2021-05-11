use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::{self, Regex};
use sodiumoxide::crypto::pwhash::argon2id13;

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

pub fn verify_hash(passwd: &str, pwh: &str) -> bool {
    sodiumoxide::init().unwrap();

    let hp = argon2id13::HashedPassword::from_slice(pwh.as_bytes()).unwrap();
    argon2id13::pwhash_verify(&hp, passwd.as_bytes())
}

pub fn gen_token() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

pub fn check_cmd_syntax(s: &str) -> bool {
    let re: Regex = Regex::new(r"^([A-Za-z]+)$|^(\d+)$").unwrap();

    re.is_match(&s)
}
