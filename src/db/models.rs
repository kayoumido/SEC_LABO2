use super::schema::users;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub secret_2fa: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}
