#[macro_use] extern crate rocket;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use rocket::response::content::RawHtml;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/Blog")]
fn blog() -> RawHtml::<String> {
    let path: &Path = &Path::new("../src/pages/BlogPage.html");
    let mut html: String = String::new();
    File::open(path).unwrap().read_to_string(&mut html).unwrap();
    RawHtml::<String>(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, blog])
}