use rocket::Rocket;

mod fence;
mod stamps;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/v1/fence", routes![fence::enter, fence::exit]).mount("/api/v1/stamps",
                                                                            routes![stamps::get])
}
