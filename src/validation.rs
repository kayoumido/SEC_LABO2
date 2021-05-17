use lazy_static::lazy_static;
use regex::{self, Regex};

/// Check if a given email has the correct format (i.e. correct syntax)
/// i.e. something@somthing.something
///
/// # Arguments
///
/// * `email` - the &str to check if it's a valid email
///
pub fn is_email_valid(email: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^[a-zA-Z0-9_]+(?:.[a-zA-Z0-9_-]+)*@(?:[a-zA-Z0-9-]+\.)+[a-zA-Z]{2,7}$")
                .unwrap();
    };

    RE.is_match(email)
}

/// Check if a given password respects the apps password policy
/// i.e. it's at least 8 characters long and shorter than 64
///
/// # Arguments
///
/// * `password` - password to check if it respects the policy
///
pub fn is_password_valid(password: &str) -> bool {
    (8..65).contains(&password.len())
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        expected,
        case("doran.kayoumi@heig-vd.ch", true),
        case("dorankayoumi@gmail.com", true),
        case("k3v1n-th3-pgm@gmail.com", true),
        case("person@organisation.co.uk", true),
        case("invalidemail", false),
        case("email@email", false),
        case("@email.lo", false),
        ::trace
    )]
    fn test_valid_email_format(input: &str, expected: bool) {
        assert_eq!(is_email_valid(input), expected);
    }

    #[rstest(
        input,
        expected,
        case("verySecurePassword", true),
        case("DK7jqu5SXWeYwg$C", true),
        case("!%3T!Xd6", true),
        case("!%3T!X d6", true),
        case("パスワード 柔道", true),
        case(
            "oufxdHfqd2emvQpsfkZh3iH8Z6KHnniqj8qRpHh!f#G#jC$kwsTS*tNmYyM8tcxY",
            true
        ),
        case(
            "zN5#yM^3!Jqm#RJX#e*QA^5Au*&UnArDCvPLBoX&3v*7zxeJET%arkEmpQe@5npSx",
            false
        ),
        case("badpwd", false),
        ::trace
    )]
    fn test_if_password_respects_policy(input: &str, expected: bool) {
        assert_eq!(is_password_valid(input), expected);
    }
}
