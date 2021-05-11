use chrono::prelude::*;

use super::schema::users;

#[derive(Queryable, Debug, AsChangeset)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub secret_2fa: Option<String>,
    pub reset_token: Option<String>,
    pub reset_token_created_at: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn is_2fa_enabled(&self) -> bool {
        self.secret_2fa != None
    }

    pub fn get_2fa_secret(&self) -> Option<String> {
        if let Some(s) = self.secret_2fa.clone() {
            Some(s)
        } else {
            None
        }
    }

    pub fn get_reset_token(&self) -> Option<String> {
        if let Some(s) = self.secret_2fa.clone() {
            Some(s)
        } else {
            None
        }
    }

    pub fn set_reset_token(&mut self, token: &str) {
        self.reset_token = Some(token.to_string());
        self.reset_token_created_at = Some(Utc::now().to_rfc3339());
    }
}
