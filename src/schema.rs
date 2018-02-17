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
