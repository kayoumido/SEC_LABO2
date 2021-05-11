use crate::db::{models::NewUser, repository};
use crate::errors::AuthError;
use crate::utils;
use crate::validation::{is_email_valid, is_password_valid};

///
///
/// # Arguments
///
/// * `email`
///
/// * `password`
///
/// EXPLAIN HOW TO TEST WHEN USING MOCK
pub fn register(email: &str, password: &str) -> Result<(), AuthError> {
    if !is_email_valid(email) {
        return Err(AuthError::InvalidEmail);
    }

    if let Ok(_) = repository::get_user(email) {
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

    let new_u = repository::create_user(&u);
    if let Err(_) = new_u {
        return Err(AuthError::RegistrationError);
    }

    Ok(())
}
