#[macro_use]
extern crate diesel;

extern crate dotenv;

use read_input::prelude::*;

mod db;

#[path = "auth/auth.rs"]
mod auth;
mod errors;
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
    // let mut database = Vec::<u32>::new();

    let u = db::models::NewUser {
        email: "dorankayoumi@gmail.com",
        password: "pass",
    };

    db::create_user(&u);

    if let Err(e) = db::get_user("ddorankayoumi@gmail.com") {
        println!("{}", e);
    }

    // println!("{:?}", f);

    if let Err(e) = login("aa", "aa") {
        println!("{}", e);
    }

    if let Err(e) = register("aa", "bb") {
        println!("{}", e);
    }
    println!("Hello, world!");
}
