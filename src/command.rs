/*!
 * Here lays the enums for the different commands a user can input.
 *
 * # Note
 * To simplify the serialization n' stuff, the crates `strum` & `strum_macros`
 * were used (thank you SEC Midterm :D)
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

use strum_macros::EnumString;

#[derive(PartialEq, Debug, EnumString)]
pub enum LoginScreenCmd {
    #[strum(serialize = "Login", serialize = "login", serialize = "1")]
    Login,
    #[strum(serialize = "Register", serialize = "register", serialize = "2")]
    Register,
    #[strum(serialize = "Reset", serialize = "reset", serialize = "3")]
    Reset,
    #[strum(serialize = "Quit", serialize = "quit", serialize = "4")]
    Quit,
}

#[derive(PartialEq, Debug, EnumString)]
pub enum UserProfileCmd {
    #[strum(
        serialize = "Enable two factor",
        serialize = "enable two factor",
        serialize = "1"
    )]
    Enable2FA,

    #[strum(
        serialize = "Disable two factor",
        serialize = "disable two factor",
        serialize = "2"
    )]
    Disable2FA,

    #[strum(serialize = "Logout", serialize = "logout", serialize = "3")]
    Logout,
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use std::str::FromStr;
    use strum;

    use super::*;

    #[rstest(
        input,
        expected,
        case("Login", Ok(LoginScreenCmd::Login)),
        case("login", Ok(LoginScreenCmd::Login)),
        case("1", Ok(LoginScreenCmd::Login)),
        case("Register", Ok(LoginScreenCmd::Register)),
        case("register", Ok(LoginScreenCmd::Register)),
        case("2", Ok(LoginScreenCmd::Register)),
        case("Reset", Ok(LoginScreenCmd::Reset)),
        case("reset", Ok(LoginScreenCmd::Reset)),
        case("3", Ok(LoginScreenCmd::Reset)),
        case("Quit", Ok(LoginScreenCmd::Quit)),
        case("quit", Ok(LoginScreenCmd::Quit)),
        case("4", Ok(LoginScreenCmd::Quit)),
        case("UnknownCmd", Err(strum::ParseError::VariantNotFound)),
        case("5", Err(strum::ParseError::VariantNotFound)),
        ::trace
    )]
    fn test_login_screen_cmd_from_string(
        input: &str,
        expected: Result<LoginScreenCmd, strum::ParseError>,
    ) {
        assert_eq!(LoginScreenCmd::from_str(input), expected);
    }

    #[rstest(
        input,
        expected,
        case("Enable two factor", Ok(UserProfileCmd::Enable2FA)),
        case("enable two factor", Ok(UserProfileCmd::Enable2FA)),
        case("1", Ok(UserProfileCmd::Enable2FA)),
        case("Disable two factor", Ok(UserProfileCmd::Disable2FA)),
        case("disable two factor", Ok(UserProfileCmd::Disable2FA)),
        case("2", Ok(UserProfileCmd::Disable2FA)),
        case("Logout", Ok(UserProfileCmd::Logout)),
        case("logout", Ok(UserProfileCmd::Logout)),
        case("3", Ok(UserProfileCmd::Logout)),
        case("UnknownCmd", Err(strum::ParseError::VariantNotFound)),
        case("5", Err(strum::ParseError::VariantNotFound)),
        ::trace
    )]
    fn test_user_profile_cmd_from_string(
        input: &str,
        expected: Result<UserProfileCmd, strum::ParseError>,
    ) {
        assert_eq!(UserProfileCmd::from_str(input), expected);
    }
}
