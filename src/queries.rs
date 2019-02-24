pub mod channels {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::QueryResult;
    use diesel::ExpressionMethods;
    use diesel::insert_into;
    use crate::schema::channels;
    use crate::models::*;

    pub fn get(conn: &PgConnection, id: i32) -> QueryResult<Option<Channel>> {
        channels::table
            .filter(channels::id.eq(id))
            .first::<Channel>(conn)
            .optional()
    }

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

    pub fn get_all_for(conn: &PgConnection, user_id: i32) -> QueryResult<Vec<Channel>> {
        use crate::schema::subscriptions;
        channels::table
            .inner_join(subscriptions::table)
            .filter(subscriptions::columns::user_id.eq(user_id))
            .select(channels::all_columns)
            .get_results(conn)
    }
}

pub mod items {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::QueryResult;
    use diesel::ExpressionMethods;
    use diesel::insert_into;
    use crate::schema::items;
    use crate::models::*;

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
    pub fn get_all_for(conn: &PgConnection, user_id: i32) -> QueryResult<Vec<UserItem>> {
        use crate::schema::read_items;
        use crate::schema::subscriptions;

        items::table
            .inner_join(subscriptions::table.on(items::channel_id.eq(subscriptions::channel_id)))
            .left_join(read_items::table)
            .filter(subscriptions::user_id.eq(user_id))
            .select((
                items::all_columns,
                read_items::all_columns.nullable(),
            ))
            .get_results(conn)
            .map(|v: Vec<(Item, Option<ReadItem>)>| {
                v.into_iter()
                    .map(|(i, r)| UserItem {
                        item: i,
                        read: r.is_some(),
                    })
                    .collect()
            })
    }
}

pub mod users {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::QueryResult;
    use diesel::ExpressionMethods;
    use diesel::insert_into;
    use crate::schema::users;
    use crate::models::*;

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
    use crate::schema::subscriptions;
    use crate::models::*;

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
    use crate::schema::read_items;
    use crate::schema::items;
    use crate::models::*;

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

    pub fn read(conn: &PgConnection, user_id: i32, item_id: i32) -> QueryResult<()> {
        get_or_create(conn, &NewReadItem {user_id, item_id}).map(|_| ())
    }

    pub fn read_all(conn: &PgConnection, user_id: i32) -> QueryResult<()> {
        let unread = items::table
            .left_join(read_items::table)
            .filter(read_items::id.is_null())
            .select(items::id)
            .get_results(conn)?;
        println!("{:?}", unread);
        for item_id in unread {
            create(conn, &NewReadItem{ user_id, item_id })?;
        }

        Ok(())
    }
}
