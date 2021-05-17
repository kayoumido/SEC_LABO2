/*!
 * Database configurations
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

pub mod models;
pub mod repository;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

/// Establish a connection to a SQLite database with the url set in a `.env` file
fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
