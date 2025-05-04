mod mac_address;
mod magic_packet;

#[macro_use] extern crate rocket;

use std::net::IpAddr;
use rocket::fs::NamedFile;
use std::path::Path;
use rocket::form::Form;
use crate::magic_packet::magic;

#[derive(Debug)]
#[derive(FromForm)]
struct WakeRequest {
    mac: mac_address::MacAddress,
    broadcast: IpAddr,
    port: usize
}

#[catch(422)]
fn bad_request() -> &'static str {
    "Wake request is not well formed"
}

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open("static/index.html").await.ok().unwrap()
}

#[get("/pico.amber.min.css")]
async fn pico() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pico.amber.min.css")).await.ok()
}

#[get("/wakeonweb.js")]
async fn js() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/wakeonweb.js")).await.ok()
}

#[post("/wake", data="<wake_request>")]
async fn wake(wake_request: Form<WakeRequest>) -> Result<String, String> {
    magic(wake_request.into_inner()).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, pico, js])
        .mount("/api", routes![wake])
        .register("/", catchers![bad_request])
}

// fn mac_validate<'v>(mac: &'v str) -> rocket::form::Result<'v, ()> {
//
// }