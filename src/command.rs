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
pub enum MenuCmd {
    #[strum(serialize = "Login", serialize = "login", serialize = "1")]
    Login,
    #[strum(serialize = "Register", serialize = "register", serialize = "2")]
    Register,
    #[strum(serialize = "Quit", serialize = "quit", serialize = "3")]
    Quit,
}
