use diesel::{insert_into, prelude::*, update};

use super::establish_connection;
use super::models::*;
use super::schema::users::dsl::*;

use crate::errors::UserDBError;

pub trait UserRepository {
    fn get_user(&self, e: &str) -> Result<User, UserDBError>;
    fn create_user(&self, e: &str, passwd: &str) -> Result<(), UserDBError>;
    fn update_user(&self, u: &User) -> Result<(), UserDBError>;
}

pub struct SQliteUserRepository {}

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
