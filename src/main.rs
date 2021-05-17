/*!
 * Second labratory for the Secure Coding course at the HEIG-VD.
 *
 * This project (crate?) is an implementation of a simple authentication system.
 * Features:
 *  - Login
 *  - Registration
 *  - Reset password
 *  - Enable/Disable 2FA
 *
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 *
 * # Note
 * Some functions weren't tested because they require mocking and/or were using
 * the `sodiumoxide` which for some reason doesn't like to be tested.
 * Details on how they would be tested can be found in the README.md
 */

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod auth;
mod command;
mod db;
mod errors;
mod process;
mod user_input;
mod utils;
mod validation;

use db::models::User;

fn login_screen() {
    println!();
    println!("Login screen");
    println!("---------");
    println!("1. Login");
    println!("2. Register");
    println!("3. Reset password");
    println!("4. Quit");
}

fn user_profile_screen(user_email: &str) {
    println!();
    println!("{}' profile", user_email);
    println!("---------");
    println!("1. Enable two factor authentication");
    println!("2. Disable two factor authentication");
    println!("3. Logout");
}

fn main() {
    // Login screen
    let mut authenticated_user: User;
    loop {
        login_screen();
        match user_input::ask_for_login_screen_cmd() {
            command::LoginScreenCmd::Login => {
                authenticated_user = process::login_process();
                break;
            }
            command::LoginScreenCmd::Register => {
                process::registration_process();
                // change?
                authenticated_user = process::login_process();
                break;
            }
            command::LoginScreenCmd::Reset => process::reset_password_process(),
            command::LoginScreenCmd::Quit => return,
        }
    }

    // Profil screen
    loop {
        user_profile_screen(&authenticated_user.get_email());
        match user_input::ask_for_user_profile_cmd() {
            command::ProfileScreenCmd::Enable2FA => {
                process::enable_2fa_process(&mut authenticated_user)
            }
            command::ProfileScreenCmd::Disable2FA => {
                process::disable_2fa_process(&mut authenticated_user)
            }
            command::ProfileScreenCmd::Logout => break,
        }
    }
}
