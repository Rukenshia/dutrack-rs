use db::Database;
use uuid::Uuid;

use log::LOGGER;

use db::models::*;
use db::schema::stamps;

use diesel::prelude::*;
use diesel;

use chrono::NaiveDateTime;
use chrono::prelude::UTC;

#[derive(Serialize)]
pub struct PublicStamp {
    pub id: String,
    pub fence: String,
    pub event: String,
    pub time: NaiveDateTime,
}

impl PublicStamp {
    pub fn from_stamp(s: &Stamp) -> Self {
        PublicStamp {
            id: format!("{}", s.id),
            fence: format!("{}", s.fence),
            event: s.event.clone(),
            time: s.time,
        }
    }
}

pub enum FenceEvent {
    Enter,
    Exit,
}

impl FenceEvent {
    pub fn as_str(&self) -> &str {
        match self {
            &FenceEvent::Enter => "enter",
            &FenceEvent::Exit => "exit",
        }
    }
}

impl Stamp {
    pub fn create(fence: &Uuid, ev: FenceEvent) -> Result<Self, String> {
        let new_stamp = NewStamp {
            fence: fence.clone(),
            event: ev.as_str().into(),
            time: UTC::now().naive_utc(),
        };

        let con = Database::get().pg.lock().unwrap();
        debug!(LOGGER,
               "creating stamp {} for fence {} AT {}",
               ev.as_str(),
               fence,
               UTC::now().naive_utc());
        match diesel::insert(&new_stamp).into(stamps::table).get_result::<Stamp>(&*con) {
            Ok(s) => Ok(s),
            Err(e) => {
                error!(LOGGER, "could not create fence: {}", e);
                Err(format!("db: {}", e))
            }
        }
    }

    pub fn by_id(stamp_id: &Uuid) -> Result<Self, String> {
        use db::schema::stamps::dsl::*;
        let con = Database::get().pg.lock().unwrap();

        match stamps.find(stamp_id).first::<Stamp>(&*con) {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("db: {}", e)),
        }
    }
}

impl PartialEq<String> for FenceEvent {
    fn eq(&self, other: &String) -> bool {
        return self.as_str() == other;
    }
    fn ne(&self, other: &String) -> bool {
        return self.as_str() != other;
    }
}