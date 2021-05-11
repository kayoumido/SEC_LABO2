use chrono::prelude::*;

use super::schema::users;

#[derive(Queryable, Debug, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
pub struct User {
    id: i32,
    email: String,
    password: String,
    secret_2fa: Option<String>,
    reset_token: Option<String>,
    reset_token_created_at: Option<String>,
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

    // GETTERS & SETTERS

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_string()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn set_password(&mut self, passwd: &str) {
        self.password = passwd.to_string()
    }

    pub fn get_secret_2fa(&self) -> Option<String> {
        if let Some(s) = self.secret_2fa.clone() {
            Some(s)
        } else {
            None
        }
    }

    pub fn set_secret_2fa(&mut self, secret: Option<String>) {
        self.secret_2fa = secret;
    }

    pub fn get_reset_token(&self) -> Option<String> {
        if let Some(s) = self.reset_token.clone() {
            Some(s)
        } else {
            None
        }
    }

    pub fn set_reset_token(&mut self, token: &str) {
        self.reset_token = Some(token.to_string());
        self.reset_token_created_at = Some(Utc::now().to_rfc3339());
    }

    pub fn get_reset_token_created_at(&self) -> Option<String> {
        if let Some(s) = self.reset_token_created_at.clone() {
            Some(s)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::User;

    /**
     * Note: Only the "complicated" functions were tested.
     *       This means that getters/setters for simple fields aren't tested
     *       e.g. get_email & set_email
     */

    #[test]
    fn test_is_2fa_enabled() {
        let mut dummy = User {
            id: 1,
            email: "dummy@test.lo".to_string(),
            password: "hashedpasswd".to_string(),
            secret_2fa: Some("2fasecret".to_string()),
            reset_token: None,
            reset_token_created_at: None,
        };

        assert_eq!(dummy.is_2fa_enabled(), true);

        dummy.set_secret_2fa(None);
        assert_eq!(dummy.is_2fa_enabled(), false);
    }

    #[test]
    fn test_set_reset_token() {
        let mut dummy = User {
            id: 1,
            email: "dummy@test.lo".to_string(),
            password: "hashedpasswd".to_string(),
            secret_2fa: Some("2fasecret".to_string()),
            reset_token: None,
            reset_token_created_at: None,
        };

        assert_eq!(dummy.get_reset_token(), None);
        assert_eq!(dummy.get_reset_token_created_at(), None);

        dummy.set_reset_token("token");

        assert_ne!(dummy.get_reset_token(), None);
        assert_ne!(dummy.get_reset_token_created_at(), None);
    }
}
