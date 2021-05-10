#[macro_use]
extern crate diesel;
extern crate dotenv;

use read_input::prelude::*;

#[path = "auth/auth.rs"]
mod auth;
mod db;
mod errors;
mod utils;
mod validation;

use auth::{login, register, twofa};

fn ask_for_email() -> String {
    input()
        .repeat_msg("Email : ")
        // .add_err_test(
        //     move |m: &String| valid_email_format(m),
        //     "Invalid mail address, please try again",
        // )
        .get()
}

fn ask_for_password() -> String {
    input()
        .repeat_msg("Password : ")
        // .add_err_test(
        //     move |m: &String| valid_password(m),
        //     "Password length must be between 8 and 64, please try again",
        // )
        .get()
}

fn main() {
    if let Err(e) = register::register("dorankayoumi@gmail.com", "password") {
        println!("{}", e);
    }

    let u = login::login("dorankayoumi@gmail.com", "password");
    if let Err(e) = u {
        println!("{}", e);
    } else {
        let mut u = u.unwrap();
        if !u.is_2fa_enabled() {
            let secret = twofa::generate_secret();

            let qr_url = twofa::generate_qr(&secret, &u.email, "Lab 02 - Authentication");
            println!("{}", qr_url);

            let code: String = input()
                .msg("To validate 2FA setup, enter the auth code: ")
                .get();

            if twofa::check_code(&secret, &code) {
                // in a real world situation, the secret should be ciphered before being add to the to db
                u.secret_2fa = Some(secret);
                db::update_user(&u);
            } else {
                println!("2fa error");
            }
        }
    }

    println!("finish");
    // setup_2fa
}
