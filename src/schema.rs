table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        domain -> Nullable<Varchar>,
        channel_id -> Int4,
    }
}

table! {
    channels (id) {
        id -> Int4,
        title -> Varchar,
        link -> Varchar,
        source -> Varchar,
        description -> Varchar,
        language -> Nullable<Varchar>,
        copyright -> Nullable<Varchar>,
        pub_date -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        ttl -> Nullable<Int4>,
    }
}

table! {
    items (id) {
        id -> Int4,
        channel_id -> Int4,
        title -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        author -> Nullable<Varchar>,
        guid -> Nullable<Varchar>,
        pub_date -> Nullable<Varchar>,
    }
}

table! {
    items_categories (id) {
        id -> Int4,
        item_id -> Int4,
        category_id -> Int4,
    }
}

joinable!(categories -> channels (channel_id));
joinable!(items -> channels (channel_id));
joinable!(items_categories -> categories (category_id));
joinable!(items_categories -> items (item_id));

allow_tables_to_appear_in_same_query!(
    categories,
    channels,
    items,
    items_categories,
);
