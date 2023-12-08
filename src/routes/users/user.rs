use rocket::{delete, get, post, put, routes, Route};

#[get("/users")]
pub fn user_list() -> String {
    "List of users".to_string()
}

#[post("/users")]
pub fn new_user() -> String {
    "Creation of new user".to_string()
}

#[get("/users/<id>")]
pub fn info_user(id: String) -> String {
    format!("Info for user {}", id)
}

#[put("/users/<id>")]
pub fn update_user(id: String) -> String {
    format!("Update info for user {}", id)
}

#[delete("/users/<id>")]
pub fn delete_user(id: String) -> String {
    format!("Delete user {}", id)
}

pub fn get_routes() -> Vec<Route> {
    routes![user_list, new_user, delete_user, info_user, update_user,]
}
