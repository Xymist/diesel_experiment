use rocket_contrib::Json;
use schema::users;
use diesel::prelude::*;

#[derive(Identifiable, Debug, Queryable, PartialEq, Serialize, Deserialize, Associations)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub username: String,
    pub email: String,
}

#[get("/users")]
fn index(db: ::database::DB) -> Json<Vec<User>> {
    use schema::users::dsl::*;
    Json(
        users
            .limit(5)
            .load::<User>(db.conn())
            .expect("Error loading users"),
    )
}

#[get("/users/<user_id>")]
fn show(user_id: i64, db: ::database::DB) -> Json<User> {
    use schema::users::dsl::*;
    Json(
        users
            .find(user_id)
            .get_result::<User>(db.conn())
            .expect("Error loading user"),
    )
}
