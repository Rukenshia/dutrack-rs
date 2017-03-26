use uuid::Uuid;
use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::JSON;
use lib::db::models::{User, Stamp, Workday};
use lib::stamp::PublicStamp;

use chrono::NaiveDate;

#[derive(Serialize)]
pub struct PublicWorkday {
    id: String,
    fence: String,
    date: NaiveDate,
    stamps: Vec<PublicStamp>,
}

impl PublicWorkday {
    pub fn from_workday(w: &Workday, stamps: Vec<Stamp>) -> Self {
        PublicWorkday {
            id: format!("{}", w.id),
            fence: format!("{}", w.fence),
            date: w.date.clone(),
            stamps: stamps.into_iter().map(|s| PublicStamp::from_stamp(&s)).collect(),
        }
    }
}

#[get("/today")]
#[allow(unused)]
pub fn get_today(u: User) -> Result<JSON<PublicWorkday>, Failure> {
    match Workday::today(&u.fence_key) {
        Ok(w) => {
            let stamps = match w.get_stamps() {
                Ok(s) => s,
                Err(_) => return Err(Failure(Status::InternalServerError)),
            };

            Ok(JSON(PublicWorkday::from_workday(&w, stamps)))
        }
        Err(_) => Err(Failure(Status::NotFound)),
    }
}

#[get("/<workday>", rank = 2)]
#[allow(unused)]
pub fn get(workday: &str, u: User) -> Result<JSON<PublicWorkday>, Failure> {
    let id = match Uuid::parse_str(workday) {
        Ok(i) => i,
        Err(_) => return Err(Failure(Status::BadRequest)),
    };

    match Workday::by_id(&id) {
        Ok(w) => {
            let stamps = match w.get_stamps() {
                Ok(s) => s,
                Err(_) => return Err(Failure(Status::InternalServerError)),
            };

            Ok(JSON(PublicWorkday::from_workday(&w, stamps)))
        }
        Err(_) => Err(Failure(Status::NotFound)),
    }
}

#[derive(FromForm)]
pub struct Filter<'a> {
    pub date: &'a str,
}

#[get("/?<filter>", rank = 2)]
#[allow(unused)]
pub fn get_by_date(filter: Filter, u: User) -> Result<JSON<PublicWorkday>, Failure> {
    let nd = match NaiveDate::parse_from_str(filter.date, "%Y-%m-%d") {
        Ok(n) => n,
        Err(_) => return Err(Failure(Status::BadRequest)),
    };

    match Workday::by_date(&u.fence_key, &nd) {
        Ok(w) => {
            let stamps = match w.get_stamps() {
                Ok(s) => s,
                Err(_) => return Err(Failure(Status::InternalServerError)),
            };

            Ok(JSON(PublicWorkday::from_workday(&w, stamps)))
        }
        Err(_) => Err(Failure(Status::NotFound)),
    }
}
