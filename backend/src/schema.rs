table! {
    plants (id) {
        id -> Int4,
        uid -> Varchar,
        symbol -> Varchar,
        sci_name -> Varchar,
        gener_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        uid -> Varchar,
        username -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    plants,
    users,
);