use crate::db::repository::{SQliteUserRepository, UserRepository};
use crate::errors::AuthError;
use crate::utils;
use crate::validation::{is_email_valid, is_password_valid};

pub fn register(email: &str, password: &str) -> Result<(), AuthError> {
    let repository = SQliteUserRepository {};
    _register(email, password, &repository)
}

///
///
/// # Arguments
///
/// * `email`
///
/// * `password`
///
/// EXPLAIN HOW TO TEST WHEN USING MOCK
fn _register(
    email: &str,
    password: &str,
    repository: &dyn UserRepository,
) -> Result<(), AuthError> {
    if !is_email_valid(email) {
        return Err(AuthError::InvalidEmail);
    }

    if let Ok(_) = repository.get_user(email) {
        return Err(AuthError::EmailUsed);
    }

    if !is_password_valid(password) {
        return Err(AuthError::InvalidPassword);
    }

    let pwh = utils::hash(password);

    let new_u = repository.create_user(email, &pwh);
    if let Err(_) = new_u {
        return Err(AuthError::RegistrationError);
    }

    Ok(())
}
