/*!
 * All of the repositories to interact with the storage
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

use diesel::{insert_into, prelude::*, update};

use super::establish_connection;
use super::models::*;
use super::schema::users::dsl::*;

use crate::errors::UserDBError;

pub trait UserRepository {
    /// Try and get a user from the storage
    /// if the wanted user doesn't exist, an error is returned
    ///
    /// # Arguments
    ///
    /// * `e` - email of the user to retrieve
    ///
    fn get_user(&self, e: &str) -> Result<User, UserDBError>;

    /// Try and create a new user in the storage
    /// if something goes wrong, an error is returned
    ///
    /// # Arguments
    ///
    /// * `e` - email of the new user
    /// * `passwd` - password of the new user
    ///
    fn create_user(&self, e: &str, passwd: &str) -> Result<(), UserDBError>;

    /// Try and update an existing user in the storage
    /// if something goes wrong, an error is returned
    ///
    /// # Arguments
    ///
    /// * `u` - the user object containing all the information (changed or unchanged)
    ///
    fn update_user(&self, u: &User) -> Result<(), UserDBError>;
}

pub struct SQliteUserRepository {}

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
/// Implementation of the `UserRepository` with SQLite as a storage
impl UserRepository for SQliteUserRepository {
    fn get_user(&self, e: &str) -> Result<User, UserDBError> {
        let conn = establish_connection();
        let res = users.filter(email.eq(e)).first::<User>(&conn);

        if let Err(_) = res {
            Err(UserDBError::GetUserError)
        } else {
            Ok(res.unwrap())
        }
    }

    fn create_user(&self, e: &str, passwd: &str) -> Result<(), UserDBError> {
        let u = NewUser {
            email: e,
            password: passwd,
        };

        let conn = establish_connection();
        if let Err(_) = insert_into(users).values(u).execute(&conn) {
            return Err(UserDBError::CreateUserError);
        }

        Ok(())
    }

    fn update_user(&self, u: &User) -> Result<(), UserDBError> {
        let conn = establish_connection();
        if let Err(_) = update(users.filter(id.eq(u.get_id())))
            .set(u)
            .execute(&conn)
        {
            return Err(UserDBError::UpdateUserError);
        }

        Ok(())
    }
}
