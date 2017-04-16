use rocket::Rocket;
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

use lib::db::models::User;
use user::FrontendUser;

#[derive(Serialize)]
struct SetupContext {
    user: FrontendUser,
    enter_uri: String,
    exit_uri: String,
}

#[get("/")]
fn setup(user: User) -> Template {
    let data = SetupContext {
        user: FrontendUser::from_user(&user),
        enter_uri: format!("http://time.in.fkn.space/api/v1/fence/{}/enter",
                           user.fence_key),
        exit_uri: format!("http://time.in.fkn.space/api/v1/fence/{}/exit",
                          user.fence_key),
    };
    Template::render("setup/index", &data)
}

#[get("/", rank = 2)]
fn setup_redir() -> Redirect {
    Redirect::to("/")
}

#[get("/finish")]
fn finish_setup(mut user: User) -> Flash<Redirect> {
    match user.finish_setup() {
        Ok(_) => Flash::success(Redirect::to("/"), ""),
        Err(e) => Flash::error(Redirect::to("/500"), e),
    }
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/setup", routes![setup, setup_redir, finish_setup])
}
