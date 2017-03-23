use uuid::Uuid;
use rocket::response::Failure;
use rocket::http::Status;
use dutrack_lib::db::models::User;
use dutrack_lib::db::models::Stamp;
use dutrack_lib::stamp::FenceEvent;

#[get("/<fence>/enter")]
pub fn enter(fence: &str) -> Result<(), Failure> {
    let id = match Uuid::parse_str(fence) {
        Ok(i) => i,
        Err(_) => return Err(Failure(Status::InternalServerError)),
    };

    let user = match User::from_fence(&id) {
        Ok(u) => u,
        Err(_) => return Err(Failure(Status::NotFound)),
    };

    match Stamp::create(&id, FenceEvent::Enter) {
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

    let user = match User::from_fence(&id) {
        Ok(u) => u,
        Err(_) => return Err(Failure(Status::NotFound)),
    };


    match Stamp::create(&id, FenceEvent::Exit) {
        Ok(_) => Ok(()),
        Err(_) => Err(Failure(Status::InternalServerError)),
    }
}