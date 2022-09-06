use std::net::{Ipv4Addr};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str { "this is root" }

#[get("/echo/<echo>")]
fn get_echo(echo: &str) -> &str { echo }

#[post("/hello")]
fn post_hello() -> &'static str { "hello world?" }

#[launch]
fn rocket() -> _ { 
    let mut config = rocket::config::Config::default();
    config.address = Ipv4Addr::new(0, 0, 0, 0).into();
    config.port = 30000;
    rocket::build().configure(config).mount("/", routes![index, get_echo, post_hello])
}
