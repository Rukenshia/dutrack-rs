use rocket::Rocket;

mod fence;
mod stamps;
mod workday;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/v1/fence", routes![fence::enter, fence::exit])
        .mount("/api/v1/stamps", routes![stamps::get])
        .mount("/api/v1/workdays",
               routes![workday::get, workday::get_today, workday::get_by_date])
}
