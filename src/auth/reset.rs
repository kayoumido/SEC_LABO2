use chrono::prelude::*;

use crate::db::repository::{SQliteUserRepository, UserRepository};
use crate::errors::AuthError;
use crate::utils;

const CODE_VALIDITY_MIN: i64 = 15;

pub fn generate_reset_token(email: &str) -> Result<(), AuthError> {
    let repository = SQliteUserRepository {};
    _generate_reset_token(email, &repository)
}

pub fn change_password(email: &str, new_passwd: &str) -> Result<(), AuthError> {
    let repository = SQliteUserRepository {};
    _change_password(email, new_passwd, &repository)
}

pub fn check_token(email: &str, token: &str) -> Result<(), AuthError> {
    let repository = SQliteUserRepository {};
    _check_token(email, token, &repository)
}

pub fn send_reset_token(email: &str) {
    let repository = SQliteUserRepository {};
    _send_reset_token(email, &repository)
}

/// EXPLAIN HOW TO TEST WHEN USING MOCK
fn _generate_reset_token(email: &str, repository: &dyn UserRepository) -> Result<(), AuthError> {
    // generate the reset token
    // note: A token is generated even though the user doesn't exists
    //       this is done to not leak the info that the user doesn't exist.
    let token = utils::gen_token();

    // try and find the user in the db
    let u = repository.get_user(email);
    if let Err(_) = u {
        return Err(AuthError::ResetError);
    }

    // update the user with the reset token
    let mut u = u.unwrap();
    u.set_reset_token(&token);
    if let Err(_) = repository.update_user(&u) {
        return Err(AuthError::ResetError);
    }

    Ok(())
}

fn _change_password(
    email: &str,
    new_passwd: &str,
    repository: &dyn UserRepository,
) -> Result<(), AuthError> {
    let u = repository.get_user(email);
    if let Err(_) = u {
        return Err(AuthError::ResetError);
    }
    let mut u = u.unwrap();

    // update the users password
    u.set_password(&utils::hash(new_passwd));

    if let Err(_) = repository.update_user(&u) {
        return Err(AuthError::ResetError);
    }

    Ok(())
}

fn _check_token(
    email: &str,
    token: &str,
    repository: &dyn UserRepository,
) -> Result<(), AuthError> {
    let u = repository.get_user(email);
    if let Err(_) = u {
        return Err(AuthError::ResetError);
    }
    let u = u.unwrap();

    let token_created_at =
        DateTime::parse_from_rfc3339(u.get_reset_token_created_at().unwrap().as_str()).unwrap();
    let now = DateTime::parse_from_rfc3339(Utc::now().to_rfc3339().as_str()).unwrap();

    if (now - token_created_at).num_minutes() > CODE_VALIDITY_MIN {
        Err(AuthError::ExpiredToken)
    } else if u.get_reset_token().unwrap() != token {
        Err(AuthError::TokenMismatch)
    } else {
        Ok(())
    }
}

fn _send_reset_token(email: &str, repository: &dyn UserRepository) {
    let u = repository.get_user(email).unwrap();

    println!();
    println!("from: lab02.auth@heig-vd.lo");
    println!("to: {}", email);
    println!("subject: Lab 02 - Auth Reset token");
    println!("message:");
    println!("Here is your reset token: {}", u.get_reset_token().unwrap());
    println!("Kind regards");
    println!();
}
