use rocket::Rocket;

mod fence;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/v1/fence", routes![fence::enter, fence::exit])
}
