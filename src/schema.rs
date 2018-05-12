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

table! {
    read_items (id) {
        id -> Int4,
        user_id -> Int4,
        item_id -> Int4,
    }
}

table! {
    subscriptions (id) {
        id -> Int4,
        user_id -> Int4,
        channel_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(items -> channels (channel_id));
joinable!(read_items -> items (item_id));
joinable!(read_items -> users (user_id));
joinable!(subscriptions -> channels (channel_id));
joinable!(subscriptions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    channels,
    items,
    read_items,
    subscriptions,
    users,
);
