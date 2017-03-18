use uuid::Uuid;

#[allow(dead_code)]
#[derive(Queryable)]
pub struct User {
  pub id: Uuid,
  pub email: String,
  pub password: String,
  pub awesome: bool,
}