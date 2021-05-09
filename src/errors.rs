use std::error;
use std::fmt;
use strum::EnumMessage;
use strum_macros;

#[derive(PartialEq, Debug, strum_macros::EnumMessage)]
pub enum AuthError {
    #[strum(message = "Your login details are incorrect.")]
    LoginError,

    #[strum(message = "Something went wrong during registration.")]
    RegistrationError,

    #[strum(message = "The e-mail address you entered is invalid.")]
    InvalidEmail,

    #[strum(message = "Your password must be between 8 and 64 characters long.")]
    InvalidPassword,

    #[strum(message = "This e-mail address is already used for another account.")]
    EmailUsed,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message().unwrap())
    }
}

impl error::Error for AuthError {
    fn description(&self) -> &str {
        self.get_message().unwrap()
    }
}
