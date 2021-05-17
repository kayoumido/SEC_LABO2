/*!
 * Functions related to registrations
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

use crate::db::repository::{SQliteUserRepository, UserRepository};
use crate::errors::AuthError;
use crate::utils;
use crate::validation::{is_email_valid, is_password_valid};

/// Public function for the registration
/// See `_register` for more info
///
pub fn register(email: &str, passwd: &str) -> Result<(), AuthError> {
    let repository = SQliteUserRepository {};
    _register(email, passwd, &repository)
}

/// User registration
///
/// # Arguments
///
/// * `email` - email for the new user
///
/// * `password` - password for the new user
///
/// * `repository` - the user repository to interact with
///
fn _register(email: &str, passwd: &str, repository: &dyn UserRepository) -> Result<(), AuthError> {
    if !is_email_valid(email) {
        return Err(AuthError::InvalidEmail);
    }

    if let Ok(_) = repository.get_user(email) {
        return Err(AuthError::EmailUsed);
    }

    if !is_password_valid(passwd) {
        return Err(AuthError::InvalidPassword);
    }

    let pwh = utils::hash(passwd);

    let res = repository.create_user(email, &pwh);
    if let Err(_) = res {
        return Err(AuthError::RegistrationError);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::models::User;
    use crate::db::repository::MockSQliteUserRepository;
    use crate::errors::UserDBError;

    #[test]
    fn test_register_with_invalid_email() {
        let mut mock = MockSQliteUserRepository::new();

        mock.expect_get_user()
            .returning(|_| Err(UserDBError::GetUserError));

        let res = _register("email", "password", &mock);

        assert_eq!(Err(AuthError::InvalidEmail), res);
    }

    #[test]
    fn test_register_with_invalid_password() {
        let mut mock = MockSQliteUserRepository::new();

        mock.expect_get_user()
            .returning(|_| Err(UserDBError::GetUserError));

        let res = _register("email@test.mock", "p", &mock);

        assert_eq!(Err(AuthError::InvalidPassword), res);
    }

    #[test]
    fn test_register_with_valid_info() {
        let mut mock = MockSQliteUserRepository::new();

        mock.expect_get_user()
            .returning(|_| Err(UserDBError::GetUserError));

        mock.expect_create_user().returning(|_, _| Ok(()));

        let res = _register("email@test.mock", "password", &mock);

        assert_eq!(Ok(()), res);
    }

    #[test]
    fn test_register_with_existing_user_info() {
        let mut mock = MockSQliteUserRepository::new();

        mock.expect_get_user()
            .returning(|e| Ok(User::new(e, "passwd_hash")));

        let res = _register("email@test.mock", "password", &mock);

        assert_eq!(Err(AuthError::EmailUsed), res);
    }
}
