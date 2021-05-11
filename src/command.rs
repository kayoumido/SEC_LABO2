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
    #[strum(serialize = "Enable 2FA", serialize = "enable 2fa", serialize = "1")]
    Enable2FA,
    #[strum(serialize = "Disable 2FA", serialize = "disable 2fa", serialize = "2")]
    Disable2FA,
    #[strum(serialize = "Logout", serialize = "logout", serialize = "3")]
    Logout,
}
