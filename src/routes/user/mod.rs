use lib::db::models::{User, Stamp, Workday};
use rocket::Rocket;
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

mod login;
mod registration;

use self::login::*;
use self::registration::*;

#[derive(Serialize)]
struct IndexContext {
    user: FrontendUser,
    stamps: Vec<FrontendStamp>,
    workday: Option<FrontendWorkday>,
}

#[derive(Serialize)]
struct FrontendUser {
    email: String,
    fence_id: String,
}

#[derive(Serialize)]
struct FrontendWorkday {
    id: String,
    fence: String,
    date: String,
    stamps: Vec<String>,
}

impl FrontendUser {
    pub fn from_user(user: &User) -> Self {
        FrontendUser {
            email: user.email.clone(),
            fence_id: format!("{}", user.fence_key),
        }
    }
}

#[derive(Serialize)]
struct FrontendStamp {
    event: String,
    time: i64,
}

impl FrontendStamp {
    pub fn from_stamp(stamp: &Stamp) -> Self {
        FrontendStamp {
            event: stamp.event.clone(),
            time: stamp.time.0,
        }
    }
}

#[get("/")]
#[allow(unused)]
pub fn index(user: User) -> Result<Template, Flash<Redirect>> {
    if !user.finished_setup {
        return Err(Flash::error(Redirect::to("/setup"), ""));
    }

    let stamps: Vec<Stamp> = match user.get_stamps() {
        Ok(s) => s,
        Err(_) => return Err(Flash::error(Redirect::to("/500"), "error getting stamps")),
    };

    let ctx = IndexContext {
        user: FrontendUser::from_user(&user),
        stamps: stamps.into_iter().map(|s| FrontendStamp::from_stamp(&s)).collect(),
        workday: match Workday::today(&user.fence_key) {
            Err(_) => None,
            Ok(w) => {
                Some(FrontendWorkday {
                         id: format!("{}", w.id),
                         fence: format!("{}", w.fence),
                         date: w.date.format("%Y-%m-%d").to_string(),
                         stamps: w.stamps
                             .into_iter()
                             .map(|s| format!("{}", s))
                             .collect(),
                     })
            }
        },
    };

    Ok(Template::render("index", &ctx))
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/",
                 routes![login,
                         logout,
                         post_login,
                         register,
                         post_register,
                         login_redirect,
                         register_redirect])
}
