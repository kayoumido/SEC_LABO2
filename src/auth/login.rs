/*!
 * Functions related to login
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

use crate::db::models::User;
use crate::db::repository::{SQliteUserRepository, UserRepository};
use crate::errors::AuthError;
use crate::utils;

/// Public function for the login
/// See `_login` for more info
///
pub fn login(email: &str, passwd: &str) -> Result<User, AuthError> {
    let repository = SQliteUserRepository {};
    _login(email, passwd, &repository)
}

/// User login
///
/// # Arguments
///
/// * `email` - the email of the user trying to login
///
/// * `passwd` - the password of the user trying to login
///
/// * `repository` - the user repository to interact with
///
fn _login(email: &str, passwd: &str, repository: &dyn UserRepository) -> Result<User, AuthError> {
    // get all the user info we need from the database
    let u = repository.get_user(email);

    if let Err(_) = u {
        // to avoid timing attacks, perform a argon2 hash to "waste" time
        utils::hash(passwd);
        return Err(AuthError::LoginError);
    }

    let u = u.unwrap();
    // check the password
    if utils::verify_hash(passwd, &u.get_password()) {
        Ok(u)
    } else {
        Err(AuthError::LoginError)
    }
}
