/*!
 * Functions related to registrations
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

use crate::db::repository::{SQliteUserRepository, UserRepository};
use crate::errors::AuthError;
use crate::utils;
use crate::validation::{is_email_valid, is_password_valid};

/// Public function for the registration
/// See `_register` for more info
///
pub fn register(email: &str, passwd: &str) -> Result<(), AuthError> {
    let repository = SQliteUserRepository {};
    _register(email, passwd, &repository)
}

/// User registration
///
/// # Arguments
///
/// * `email` - email for the new user
///
/// * `password` - password for the new user
///
/// * `repository` - the user repository to interact with
///
fn _register(email: &str, passwd: &str, repository: &dyn UserRepository) -> Result<(), AuthError> {
    if !is_email_valid(email) {
        return Err(AuthError::InvalidEmail);
    }

    if let Ok(_) = repository.get_user(email) {
        return Err(AuthError::EmailUsed);
    }

    if !is_password_valid(passwd) {
        return Err(AuthError::InvalidPassword);
    }

    let pwh = utils::hash(passwd);

    let new_u = repository.create_user(email, &pwh);
    if let Err(_) = new_u {
        return Err(AuthError::RegistrationError);
    }

    Ok(())
}
