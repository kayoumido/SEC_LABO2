use crate::db::{models::User, repository};
use crate::errors::AuthError;
use crate::utils;

const CODE_VALIDITY_MIN: u8 = 15;

fn reset(email: &str) -> Result<(), AuthError> {
    // generate the reset token
    // note: A token is generated even though the user doesn't exists
    //       this is done to not leak the info that the user doesn't exist.
    let token = utils::gen_token();

    if let Err(_) = repository::get_user(email) {
        return Err(AuthError::LoginError);
    }

    // u.reset_token = Some(utils::gen_token());
    // u.reset_token_created_at = Some(Utc::now().to_string());
    // db::update_user(&u);

    Ok(())
}

fn check_token(u: User, token: &str) -> bool {
    true
}

fn send_reset_token(email: &str, token: &str) {
    println!("from: lab02.auth@heig-vd.lo");
    println!("to: {}", email);
    println!("Here is your reset token: {}", token);
}
