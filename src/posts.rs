use rocket_contrib::Json;
use schema::posts;
use serde_json;
use diesel::prelude::*;
use users::User;

#[derive(Identifiable, Debug, Queryable, Serialize, Deserialize, PartialEq, Associations)]
#[belongs_to(User)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub user_id: i64,
}

#[get("/users/<post_user_id>/posts")]
fn index(post_user_id: i64, db: ::database::DB) -> Json<Vec<Post>> {
    use schema::users::dsl::*;
    let user = users
        .find(post_user_id)
        .get_result::<User>(db.conn())
        .expect("Error loading user");

    Json(
        Post::belonging_to(&user)
            .load::<Post>(db.conn())
            .expect("Error loading posts"),
    )
}

#[get("/users/<post_user_id>/posts/<post_id>")]
fn show(post_user_id: i64, post_id: i64, db: ::database::DB) -> String {
    use schema::users::dsl::*;
    let user = users
        .find(post_user_id)
        .get_result::<User>(db.conn())
        .expect("Error loading user");

    let maybe_post = Post::belonging_to(&user)
        .find(post_id)
        .get_result::<Post>(db.conn());

    let post = match maybe_post {
        Ok(post) => serde_json::to_string(&post),
        _ => Ok(String::from("{}")),
    };

    match post {
        Ok(post_json) => post_json,
        _ => String::from("{}"),
    }
}
