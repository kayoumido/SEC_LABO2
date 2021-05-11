use diesel::{insert_into, prelude::*, update};

use super::establish_connection;
use super::models::*;
use super::schema::users::dsl::*;

use crate::errors::UserDBError;

pub fn get_user(e: &str) -> Result<User, UserDBError> {
    let conn = establish_connection();
    let res = users.filter(email.eq(e)).first::<User>(&conn);

    if let Err(_) = res {
        Err(UserDBError::GetUserError)
    } else {
        Ok(res.unwrap())
    }
}

pub fn create_user(u: &NewUser) -> Result<(), UserDBError> {
    let conn = establish_connection();
    if let Err(_) = insert_into(users).values(u).execute(&conn) {
        return Err(UserDBError::CreateUserError);
    }

    Ok(())
}

pub fn update_user(u: &User) -> Result<(), UserDBError> {
    let conn = establish_connection();
    if let Err(_) = update(users.filter(id.eq(u.get_id())))
        .set(u)
        .execute(&conn)
    {
        return Err(UserDBError::UpdateUserError);
    }

    Ok(())
}
