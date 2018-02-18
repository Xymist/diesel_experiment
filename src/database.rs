use diesel::prelude::*;
use dotenv::dotenv;
use r2d2;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use rocket;
use rocket::request::{FromRequest, Outcome};
use rocket::Outcome::{Failure, Success};
use rocket::http::Status;
use std::env;

lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<MysqlConnection>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<MysqlConnection>>);

impl DB {
    pub fn conn(&self) -> &MysqlConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = r2d2::Error;
    fn from_request(_: &'a rocket::Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}

pub fn create_db_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create pool.")
}
