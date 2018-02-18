#![feature(plugin)]
#![feature(custom_attribute)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod database;
mod posts;
mod routes;
mod schema;
mod users;

fn main() {
    rocket::ignite().mount("/", routes::routes()).launch();
}
