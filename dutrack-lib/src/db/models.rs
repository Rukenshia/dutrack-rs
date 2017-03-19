use uuid::Uuid;
use super::schema::users;

#[allow(dead_code)]
#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub fence_key: Uuid,
    pub finished_setup: bool,
    pub awesome: bool,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}
