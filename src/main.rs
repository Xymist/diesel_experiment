#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod users;

#[get("/")]
fn index() -> String {
    use schema::users::dsl::*;
    use users::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    let user = &results[0];
    format!("{}, {}", user.username, user.email)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
