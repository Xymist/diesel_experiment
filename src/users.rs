#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub username: String,
    pub email: String,
}
