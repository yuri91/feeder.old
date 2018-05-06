table! {
    channels (id) {
        id -> Int4,
        title -> Varchar,
        link -> Varchar,
        description -> Varchar,
        source -> Varchar,
        image -> Nullable<Varchar>,
        ttl -> Nullable<Int4>,
    }
}

table! {
    items (id) {
        id -> Int4,
        channel_id -> Int4,
        title -> Varchar,
        link -> Varchar,
        description -> Varchar,
        pub_date -> Timestamp,
        guid -> Nullable<Varchar>,
    }
}

joinable!(items -> channels (channel_id));

allow_tables_to_appear_in_same_query!(
    channels,
    items,
);
