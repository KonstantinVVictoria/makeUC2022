table! {
    plants (id) {
        id -> Int4,
        name -> Varchar,
        watering_freq -> Int4,
        uid -> VarChar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
