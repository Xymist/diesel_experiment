use rocket;
use users;
use posts;

pub fn routes() -> Vec<rocket::Route> {
    routes![users::index, users::show, posts::index, posts::show]
}
