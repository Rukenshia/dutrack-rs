use db::Database;
use uuid::Uuid;

use log::LOGGER;

use db::models::*;
use db::schema::workdays;

use diesel::prelude::*;
use diesel;

use chrono::NaiveDate;

impl Workday {
    pub fn create(date: NaiveDate, fence: &Uuid, stamps: Vec<Stamp>) -> Result<Self, String> {
        let new_wd = NewWorkday {
            fence: fence.clone(),
            date: date,
            stamps: stamps.into_iter().map(|s| s.id).collect(),
        };

        let con = Database::get().pg.lock().unwrap();

        debug!(LOGGER, "creating workday for fence {}", fence);
        match diesel::insert(&new_wd).into(workdays::table).get_result::<Workday>(&*con) {
            Ok(w) => Ok(w),
            Err(e) => {
                error!(LOGGER, "could not create workday: {}", e);
                Err(format!("db: {}", e))
            }
        }
    }

    pub fn add_stamp(&mut self, stamp: Stamp) -> Result<(), String> {
        use db::schema::workdays::dsl::*;

        let con = Database::get().pg.lock().unwrap();
        self.stamps.push(stamp.id);

        debug!(LOGGER, "adding stamp {} to workday {}", stamp.id, self.id);
        match diesel::update(workdays.find(&self.id))
                  .set(stamps.eq(&self.stamps))
                  .returning(id)
                  .get_result::<Uuid>(&*con) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("db: {}", e)),
        }
    }

    pub fn by_id(workday_id: &Uuid) -> Result<Self, String> {
        use db::schema::workdays::dsl::*;
        let con = Database::get().pg.lock().unwrap();

        match workdays.find(workday_id).first::<Workday>(&*con) {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("db: {}", e)),
        }
    }

    pub fn by_date(fence_key: &Uuid, wdate: &NaiveDate) -> Result<Self, String> {
        use db::schema::workdays::dsl::*;
        let con = Database::get().pg.lock().unwrap();

        match workdays.filter(fence.eq(fence_key)).filter(date.eq(wdate)).first::<Workday>(&*con) {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("db: {}", e)),
        }
    }

    pub fn today(fence_key: &Uuid) -> Result<Self, String> {
        use db::schema::workdays::dsl::*;
        use chrono::prelude::*;

        debug!(LOGGER, "retrieving todays workday for fence {}", fence_key);

        let now: Date<UTC> = UTC::today();

        let con = Database::get().pg.lock().unwrap();
        workdays.filter(fence.eq(fence_key))
            .filter(date.eq(now.naive_utc()))
            .first::<Workday>(&*con)
            .map_err(|e| format!("{}", e))
    }

    pub fn get_stamps(&self) -> Result<Vec<Stamp>, String> {
        let mut stamps: Vec<Stamp> = vec![];

        for stamp in &self.stamps {
            let actual = match Stamp::by_id(stamp) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };

            stamps.push(actual);
        }

        Ok(stamps)
    }
}