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

fn login_screen() {}

fn user_profile_screen() {}

fn main() {
    // Login screen
    let mut authenticated_user: User;
    loop {
        login_screen();

        println!("Login screen");
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
        user_profile_screen();

        println!("Welcome {}", authenticated_user.get_email());
        match user_input::ask_for_user_profile_cmd() {
            command::UserProfileCmd::Enable2FA => {
                process::enable_2fa_process(&mut authenticated_user)
            }
            command::UserProfileCmd::Disable2FA => {
                process::disable_2fa_process(&mut authenticated_user)
            }
            command::UserProfileCmd::Logout => break,
        }
    }
}
