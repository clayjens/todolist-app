// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        complete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
