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

use auth::{login, register};

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
    // println!("{:?}", f);

    if let Err(e) = register("dorankayoumi@gmail.com", "password") {
        println!("{}", e);
    }

    if let Err(e) = login("dorankayoumi@gmail.com", "password") {
        println!("{}", e);
    }

    if let Err(e) = db::get_user("dorankayoumi@gmail.com") {
        println!("{}", e);
    } else {
        println!("all gucci")
    }
}
