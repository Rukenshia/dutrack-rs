use uuid::Uuid;
use super::schema::*;
use diesel::pg::data_types::PgTimestamp;

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

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Stamp {
    pub id: Uuid,
    pub fence: Uuid,
    pub event: String,
    pub time: PgTimestamp,
}

#[derive(Insertable)]
#[table_name="stamps"]
pub struct NewStamp<'a> {
    pub fence: Uuid,
    pub event: &'a str,
}

use chrono::NaiveDate;

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Workday {
    pub id: Uuid,
    pub fence: Uuid,
    pub date: NaiveDate,
    pub stamps: Vec<Uuid>,
}

#[derive(Insertable)]
#[table_name="workdays"]
pub struct NewWorkday {
    pub fence: Uuid,
    pub date: NaiveDate,
    pub stamps: Vec<Uuid>,
}