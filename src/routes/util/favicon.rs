use rocket::http::Status;
use rocket::response::Failure;

#[get("/favicon.ico")]
pub fn get() -> Failure {
    Failure(Status::NotFound)
}
