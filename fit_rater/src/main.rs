use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[rocket::get("/<file..>", rank = 2)]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    ignite().mount("/", routes![routes::index, routes::add_item, routes::get_item, routes::update_item, routes::delete_item, static_files]).launch();
}
