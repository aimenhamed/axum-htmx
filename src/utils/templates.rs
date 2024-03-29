use askama::Template;

use crate::handlers::User;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    pub name: String,
}

#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "users-list.html")]
pub struct UsersListTemplate {
    pub users: Vec<User>,
}
