#[derive(FromForm)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(FromForm)]
pub struct RegistrationRequest {
    pub email: String,
    pub password: String,
}