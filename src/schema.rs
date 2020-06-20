table! {
    token (value) {
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