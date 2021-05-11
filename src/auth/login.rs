use crate::db::{models::User, repository};
use crate::errors::AuthError;
use crate::utils;

///
///
/// # Arguments
///
/// * `email`
///
/// * `password`
///
/// EXPLAIN HOW TO TEST WHEN USING MOCK
pub fn login(email: &str, password: &str) -> Result<User, AuthError> {
    // get all the user info we need from the database
    let u = repository::get_user(email);

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
