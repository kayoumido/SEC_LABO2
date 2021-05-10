pub mod models;
pub mod schema;

use diesel::{insert_into, prelude::*, result::Error, update};
use dotenv::dotenv;
use std::env;

use models::*;
use schema::users::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_user(e: &str) -> Result<User, Error> {
    let conn = establish_connection();
    users.filter(email.eq(e)).first::<User>(&conn)
}

pub fn create_user(u: &NewUser) -> Result<User, Error> {
    let conn = establish_connection();
    if let Err(e) = insert_into(users).values(u).execute(&conn) {
        return Err(e);
    }

    Ok(get_user(u.email).unwrap())
}

pub fn update_user(u: &User) {
    let conn = establish_connection();
    if let Err(e) = update(users.filter(id.eq(u.id))).set(u).execute(&conn) {
        println!("{}", e);
    }
}
