use user::FrontendUser;
use lib::db::models::User;

#[derive(Serialize)]
pub struct Context {
    user: Option<FrontendUser>,
}

impl Context {
    pub fn new(user: Option<User>) -> Self {
        Context {
            user: user.map(|u| FrontendUser::from_user(&u))
        }
    }
}
