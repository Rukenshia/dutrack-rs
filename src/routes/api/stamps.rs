use uuid::Uuid;
use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::JSON;
use lib::db::models::{User, Stamp};

use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct PublicStamp {
    pub id: String,
    pub fence: String,
    pub event: String,
    pub time: NaiveDateTime,
}

#[get("/<stamp>")]
#[allow(unused)]
pub fn get(stamp: &str, u: User) -> Result<JSON<PublicStamp>, Failure> {
    let id = match Uuid::parse_str(stamp) {
        Ok(i) => i,
        Err(_) => return Err(Failure(Status::BadRequest)),
    };

    match Stamp::by_id(&id) {
        Ok(s) => {
            Ok(JSON(PublicStamp {
                        id: format!("{}", s.id),
                        fence: format!("{}", s.fence),
                        event: s.event.clone(),
                        time: s.time,
                    }))
        }
        Err(_) => Err(Failure(Status::NotFound)),
    }
}