
// use diesel::table;

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
    }
}
