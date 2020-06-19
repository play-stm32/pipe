table! {
    uuid (value) {
        value -> Varchar,
        owner -> Varchar,
    }
}

table! {
    user (username) {
        username -> Varchar,
        password -> Varchar,
    }
}