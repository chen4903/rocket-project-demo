// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Text,
        create_at -> Timestamp,
    }
}
