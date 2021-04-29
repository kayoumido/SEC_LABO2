use argon2::{Config, ThreadMode, Variant, Version};

use crate::db::{create_user, get_user, models::NewUser};
use crate::errors::AuthError;
use crate::utils;
use crate::validation::{is_email_valid, is_password_valid};
///
pub fn register(email: &str, password: &str) -> Result<(), AuthError> {
    if !is_email_valid(email) {
        return Err(AuthError::InvalidEmail);
    }

    if let Ok(_) = get_user(email) {
        return Err(AuthError::EmailUsed);
    }

    if !is_password_valid(password) {
        return Err(AuthError::InvalidPassword);
    }

    // has the password
    let salt = utils::gen_salt();

    let mut config = Config::default();
    config.variant = Variant::Argon2id;

    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();

    let u = NewUser {
        email: email,
        password: &hash,
    };

    create_user(&u);

    Ok(())
}

///
///
/// # Arguments
///
/// * `email`
///
/// * `password`
///
/// # Notes
/// Since we don't know the salt, we need to get the users hashed password
/// from the db so we can check it with argon2
///
pub fn login(email: &str, password: &str) -> Result<(), AuthError> {
    // check that the given email actually exists
    let u = get_user(email);
    if let Err(_) = u {
        return Err(AuthError::LoginError);
    }

    let hash = u.unwrap().password;
    // check the password
    if argon2::verify_encoded(&hash, password.as_bytes()).unwrap() {
        Ok(())
    } else {
        Err(AuthError::LoginError)
    }
}
