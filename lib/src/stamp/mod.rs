use db::Database;
use uuid::Uuid;

use log::LOGGER;

use db::models::*;
use db::schema::stamps;

use diesel::prelude::*;
use diesel;

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
        };

        let con = Database::get().pg.lock().unwrap();
        debug!(LOGGER,
               "creating fence event {} for fence {}",
               ev.as_str(),
               fence);
        match diesel::insert(&new_stamp).into(stamps::table).get_result::<Stamp>(&*con) {
            Ok(s) => Ok(s),
            Err(e) => {
                error!(LOGGER, "could not create fence: {}", e);
                Err(format!("db: {}", e))
            }
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