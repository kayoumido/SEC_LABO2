use diesel::{insert_into, prelude::*, result::Error, update};

use super::establish_connection;
use super::models::*;
use super::schema::users::dsl::*;

pub fn get_user(e: &str) -> Result<User, Error> {
    let conn = establish_connection();
    users.filter(email.eq(e)).first::<User>(&conn)
}

pub fn create_user(u: &NewUser) -> Result<(), Error> {
    let conn = establish_connection();
    if let Err(e) = insert_into(users).values(u).execute(&conn) {
        return Err(e);
    }

    Ok(())
}

pub fn update_user(u: &User) -> Result<(), Error> {
    let conn = establish_connection();
    if let Err(e) = update(users.filter(id.eq(u.id))).set(u).execute(&conn) {
        return Err(e);
    }

    Ok(())
}
