table! {
    posts (id) {
        id -> Bigint,
        title -> Varchar,
        content -> Varchar,
        user_id -> Bigint,
    }
}

table! {
    users (id) {
        id -> Bigint,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        username -> Varchar,
        email -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
