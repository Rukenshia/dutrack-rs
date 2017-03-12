use std::path::{Path, PathBuf};

use rocket::Rocket;
use rocket::response::NamedFile;

mod favicon;

#[get("/<file..>")]
fn asset(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/", routes![favicon::get])
        .mount("/assets", routes![asset])
}