use crate::errors::AuthError;
use crate::validation::{is_email_unused, is_email_valid, is_password_valid};

///
pub fn register(email: &str, password: &str) -> Result<(), AuthError> {
    if !is_email_valid(email) {
        return Err(AuthError::InvalidEmail);
    }

    if !is_email_unused(email) {
        return Err(AuthError::EmailUsed);
    }

    if !is_password_valid(password) {
        return Err(AuthError::InvalidPassword);
    }

    Ok(())
}

///
pub fn login(email: &str, password: &str) -> Result<(), AuthError> {
    if is_email_unused(email) {
        return Err(AuthError::LoginError);
    }
    Ok(())
}
