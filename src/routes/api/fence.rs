use uuid::Uuid;
use rocket::response::Failure;
use rocket::http::Status;
use lib::db::models::{User, Stamp, Workday};
use lib::stamp::FenceEvent;

use lib::log::error;

#[get("/<fence>/enter")]
pub fn enter(fence: &str) -> Result<(), Failure> {
    let id = match Uuid::parse_str(fence) {
        Ok(i) => i,
        Err(_) => return Err(Failure(Status::InternalServerError)),
    };

    match User::from_fence(&id) {
        Ok(_) => (),
        Err(_) => return Err(Failure(Status::NotFound)),
    };

    let stamp = match Stamp::create(&id, FenceEvent::Enter) {
        Ok(s) => s,
        Err(_) => return Err(Failure(Status::InternalServerError)),
    };

    let res = match Workday::today(&id) {
        Ok(mut w) => w.add_stamp(stamp),
        Err(_) => {
            use chrono::prelude::*;

            println!("need to create a new one");
            Workday::create(UTC::today().naive_utc(), &id, vec![stamp]).map(|_| ())
        }
    };

    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(Failure(Status::InternalServerError)),
    }
}

#[get("/<fence>/exit")]
pub fn exit(fence: &str) -> Result<(), Failure> {
    let id = match Uuid::parse_str(fence) {
        Ok(i) => i,
        Err(_) => return Err(Failure(Status::InternalServerError)),
    };

    match User::from_fence(&id) {
        Ok(_) => (),
        Err(_) => return Err(Failure(Status::NotFound)),
    };

    let stamp = match Stamp::create(&id, FenceEvent::Exit) {
        Ok(s) => s,
        Err(_) => return Err(Failure(Status::InternalServerError)),
    };

    let res = match Workday::today(&id) {
        Ok(mut w) => w.add_stamp(stamp),
        Err(_) => {
            error(&format!("exit event registered, but no workday for fence {}", id));
            return Err(Failure(Status::InternalServerError));
        }
    };

    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(Failure(Status::InternalServerError)),
    }
}