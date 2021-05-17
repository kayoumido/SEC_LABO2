# SEC - Lab 02 Authentication
> Author: Doran Kayoumi

## Setup

### Diesel CLI

First things first, you need to have the [diesel cli](https://crates.io/crates/diesel_cli) installed on your machine

```bash
$ cargo install diesel_cli 
```

Next, you need to create an `.env` file. An example file is provided so you can simply copy it.

```bash
$ cp .env.example .env
```

If you want, you can change the `DATABASE_URL` to whatever you want, just update the `.env` file.

Now you can create the database by running the following

```bash
$ diesel database setup
```

And voilà!

### Manually

> :warning: I haven't tried this method so I'm just speculating on how it can be done :warning:

If you don't want to install the diesel cli, you can manually create an SQLite database.

> Note: just make sure you create the users table (see `up.sql` in create_users migration for SQL code) & setup the correct database url in the `.env`.

## Test description

Some of my code isn't tested because was using `sodiumoxide::argon2id13::pwhash_verify` which generates and error during the tests. So here is what the tests would look like if there weren't any errors generated by `sodiumoxide::argon2id13::pwhash_verify`.

### Login.rs
```rust
#[cfg(test)]
mod test {
    use super::*;
    use crate::db::repository::MockSQliteUserRepository;
    use crate::errors::UserDBError;
    
    #[test]
    fn test_login_with_known_user_and_wrong_password() {
        let mut mock = MockSQliteUserRepository::new();

        mock.expect_get_user()
            .returning(|e| Ok(User::new(e, "password")));

        let res = _login("email@email.test", "passwd", &mock);

        assert_eq!(Err(AuthError::LoginError), res);
    }

    #[test]
    fn test_login_with_known_user_and_correct_password() {
        let mut mock = MockSQliteUserRepository::new();

        mock.expect_get_user()
            .returning(|e| Ok(User::new(e, "password")));

        let res = _login("email@email.test", "password", &mock);

        assert_eq!(Ok(User::new(e, "password")), res);
    }
}

```
### Utils.rs


```rust
#[test]
fn test_verify_hash() {
    let pw1 = "passwd";
    let pw2 = "verySecurePassword";

    let pwh1 = hash(pw1);
    let pwh2 = hash(pw2);

    assert_eq!(verify_hash(&pwh1, pw1), true);
    assert_eq!(verify_hash(&pwh2, pw2), true);

    assert_eq!(verify_hash(&pwh1, pw2), true);
    assert_eq!(verify_hash(&pwh2, pw1), true);
}
```