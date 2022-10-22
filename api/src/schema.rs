use diesel::{allow_tables_to_appear_in_same_query, table};

table! {
    plants (id) {
        id -> Int4,
        name -> Varchar,
        water_frequency -> Int4,
        notes -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(plants, users,);
