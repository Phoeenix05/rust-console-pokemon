// use figment::{Figment, providers::Toml};

#[macro_use]
extern crate rocket;

// use rocket::figment::providers::Toml;

#[launch]
fn rocket() -> _ {
    // let figment = rocket::Config::figment().merge(Toml::)
    // let figment = Figment::from(rocket::Config::default()).merge(Toml::)

    rocket::build().mount("/", routes![])
}
