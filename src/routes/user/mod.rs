use dutrack_lib::key::Key;
use dutrack_lib::session::SessionManager;
use rocket::response::Redirect;
use rocket::{Rocket, State};
use rocket::http::{Cookie, Cookies};

#[get("/login")]
fn login(cookies: &Cookies, sm: State<SessionManager>) -> Redirect {
  let session_token = sm.start(&Key::new().to_string()).unwrap();
  cookies.add(Cookie::new("session_token", session_token));
  Redirect::to("/")
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/", routes![login])
}