use read_input::prelude::*;
use regex::{self, Regex};
use std::str::FromStr;

use crate::command;
use crate::validation;

pub fn ask_for_email() -> String {
    input()
        .repeat_msg("Email : ")
        .add_err_test(
            move |m: &String| validation::is_email_valid(m),
            "Invalid mail address, please try again",
        )
        .get()
}

pub fn ask_for_password() -> String {
    input().msg("Password : ").get()
}

pub fn ask_for_password_with_policy_check() -> String {
    input()
        .repeat_msg("Password : ")
        .add_err_test(
            move |m: &String| validation::is_password_valid(m),
            "Password length must be between 8 and 64, please try again",
        )
        .get()
}

pub fn ask_for_authentication_code() -> String {
    println!("Open the two-factor authentication app on your device to view your authentication code and verify your identity.");
    input().msg("Authentication code: ").get()
}

pub fn ask_for_login_screen_cmd() -> command::LoginScreenCmd {
    let err_msg = "Unknown command";
    loop {
        let input: String = input()
            .msg("What do you want to do? ")
            .add_err_test(move |x: &String| check_cmd_syntax(&x), err_msg)
            .get();

        if let Err(_) = command::LoginScreenCmd::from_str(&input) {
            println!("{}", err_msg);
            continue;
        }

        return command::LoginScreenCmd::from_str(&input).unwrap();
    }
}

pub fn ask_for_user_profile_cmd() -> command::UserProfileCmd {
    let err_msg = "Unknown command";
    loop {
        let input: String = input()
            .msg("What do you want to do? ")
            .add_err_test(move |x: &String| check_cmd_syntax(&x), err_msg)
            .get();

        if let Err(_) = command::UserProfileCmd::from_str(&input) {
            println!("{}", err_msg);
            continue;
        }

        return command::UserProfileCmd::from_str(&input).unwrap();
    }
}

pub fn ask_for_reset_token() -> String {
    input().msg("Reset token : ").get()
}

pub fn check_cmd_syntax(s: &str) -> bool {
    let re: Regex = Regex::new(r"^([A-Za-z]+)$|^(\d+)$").unwrap();

    re.is_match(&s)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input, 
        expected, 
        case("Command", true),
        case("1", true), 
        case("Login", true), 
        case("Reset", true),
        case("C0mm4nd", false) 
        ::trace
    )]
    fn test_check_cmd_syntax(input: &str, expected: bool) {
        assert_eq!(check_cmd_syntax(input), expected);
    }
}