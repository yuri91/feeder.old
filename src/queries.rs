pub mod channels {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::QueryResult;
    use diesel::ExpressionMethods;
    use diesel::insert_into;
    use schema::channels;
    use models::*;

    pub fn get_or_create(conn: &PgConnection, channel: &NewChannel) -> QueryResult<Channel> {
        channels::table
            .filter(channels::link.eq(channel.link))
            .first::<Channel>(conn)
            .optional()
            .and_then(|r| match r {
                Some(r) => Ok(r),
                None => create(conn, channel),
            })
    }

    pub fn create(conn: &PgConnection, channel: &NewChannel) -> QueryResult<Channel> {
        insert_into(channels::table)
            .values(channel)
            .get_result(conn)
    }
}

pub mod items {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::QueryResult;
    use diesel::ExpressionMethods;
    use diesel::insert_into;
    use schema::items;
    use models::*;

    pub fn insert_if_new(conn: &PgConnection, item: &NewItem) -> QueryResult<Option<Item>> {
        if item.guid.is_some() {
            items::table
                .filter(items::guid.eq(item.guid))
                .first::<Item>(conn)
                .optional()
                .and_then(|r| match r {
                    Some(_) => Ok(None),
                    None => Ok(Some(create(conn, item)?)),
                })
        } else {
            items::table
                .filter(
                    items::channel_id
                        .eq(item.channel_id)
                        .and(items::title.eq(item.title))
                        .and(items::description.eq(item.description)),
                )
                .first::<Item>(conn)
                .optional()
                .and_then(|r| match r {
                    Some(_) => Ok(None),
                    None => Ok(Some(create(conn, item)?)),
                })
        }
    }

    pub fn create(conn: &PgConnection, item: &NewItem) -> QueryResult<Item> {
        insert_into(items::table).values(item).get_result(conn)
    }
}

pub mod users {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::QueryResult;
    use diesel::ExpressionMethods;
    use diesel::insert_into;
    use schema::users;
    use models::*;

    pub fn get_or_create(conn: &PgConnection, user: &NewUser) -> QueryResult<User> {
        users::table
            .filter(users::name.eq(user.name))
            .first::<User>(conn)
            .optional()
            .and_then(|r| match r {
                Some(r) => Ok(r),
                None => create(conn, user),
            })
    }

    pub fn create(conn: &PgConnection, user: &NewUser) -> QueryResult<User> {
        insert_into(users::table).values(user).get_result(conn)
    }
}

pub mod subscriptions {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::QueryResult;
    use diesel::ExpressionMethods;
    use diesel::insert_into;
    use schema::subscriptions;
    use models::*;

    pub fn get_or_create(
        conn: &PgConnection,
        subscription: &NewSubscription,
    ) -> QueryResult<Subscription> {
        subscriptions::table
            .filter(
                subscriptions::user_id
                    .eq(subscription.user_id)
                    .and(subscriptions::channel_id.eq(subscription.channel_id)),
            )
            .first::<Subscription>(conn)
            .optional()
            .and_then(|r| match r {
                Some(r) => Ok(r),
                None => create(conn, subscription),
            })
    }

    pub fn create(
        conn: &PgConnection,
        subscription: &NewSubscription,
    ) -> QueryResult<Subscription> {
        insert_into(subscriptions::table)
            .values(subscription)
            .get_result(conn)
    }
}

pub mod read_items {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::QueryResult;
    use diesel::ExpressionMethods;
    use diesel::insert_into;
    use schema::read_items;
    use models::*;

    pub fn get_or_create(conn: &PgConnection, read_item: &NewReadItem) -> QueryResult<ReadItem> {
        read_items::table
            .filter(
                read_items::user_id
                    .eq(read_item.user_id)
                    .and(read_items::item_id.eq(read_item.item_id)),
            )
            .first::<ReadItem>(conn)
            .optional()
            .and_then(|r| match r {
                Some(r) => Ok(r),
                None => create(conn, read_item),
            })
    }

    pub fn create(conn: &PgConnection, read_item: &NewReadItem) -> QueryResult<ReadItem> {
        insert_into(read_items::table)
            .values(read_item)
            .get_result(conn)
    }
}
