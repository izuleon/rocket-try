use rocket::{fs::NamedFile, get, routes, Route};
use std::path::{Path, PathBuf};

#[get("/<file..>")]
async fn fileserver(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

pub fn get_routes() -> Vec<Route> {
    routes![fileserver]
}
