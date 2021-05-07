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

    let pwh = utils::hash(password);

    let u = NewUser {
        email: email,
        password: &pwh,
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
pub fn login(email: &str, password: &str) -> Result<(), AuthError> {
    // get all the user info we need from the database
    let u = get_user(email);

    if let Err(_) = u {
        // to avoid timing attacks, perform a argon2 hash to "waste" time
        utils::hash(password);
        return Err(AuthError::LoginError);
    }

    let u = u.unwrap();
    // check the password
    if utils::verify_hash(password, &u.password) {
        Ok(())
    } else {
        Err(AuthError::LoginError)
    }
}
