use crate::db::{create_user, get_user, models::NewUser, models::User};
use crate::errors::AuthError;
use crate::utils;
use crate::validation::{is_email_valid, is_password_valid};

///
pub fn register(email: &str, password: &str) -> Result<User, AuthError> {
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

    let new_u = create_user(&u);

    if let Err(_) = new_u {
        return Err(AuthError::RegistrationError);
    }

    Ok(new_u.unwrap())
}

///
///
/// # Arguments
///
/// * `email`
///
/// * `password`
///
pub fn login(email: &str, password: &str) -> Result<User, AuthError> {
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
        Ok(u)
    } else {
        Err(AuthError::LoginError)
    }
}
