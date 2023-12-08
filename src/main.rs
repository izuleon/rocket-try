mod routes;

use dotenv::dotenv;
use rocket::{self, Build};
use routes::{staticfiles::staticfiles, users::user};

fn rocket() -> rocket::Rocket<Build> {
    dotenv().ok();
    rocket::build()
        .mount("/static", staticfiles::get_routes())
        .mount("/user", user::get_routes())
}

#[rocket::main]
async fn main() {
    rocket().launch().await.expect("Failed to launch Rocket");
}
