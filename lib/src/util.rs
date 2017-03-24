use rocket::State;
use rocket::request::{Outcome, Request, FromRequest};

#[allow(dead_code)]
pub fn get_state<'a, 'r, T: Send + Sync>(request: &'a Request<'r>) -> Outcome<State<'a, T>, ()> {
    <State<T> as FromRequest>::from_request(request)
}
