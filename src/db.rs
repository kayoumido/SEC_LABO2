pub mod models;
pub mod schema;

use diesel::{insert_into, prelude::*, result::Error};
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

pub fn create_user(u: &NewUser) {
    let conn = establish_connection();
    if let Err(e) = insert_into(users).values(u).execute(&conn) {
        println!("{}", e);
    }
}
