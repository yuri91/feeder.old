pub mod channels {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::QueryResult;
    use diesel::ExpressionMethods;
    use diesel::insert_into;
    use schema::channels;
    use ::models::*;

    pub fn get_or_create(conn: &PgConnection, channel: &NewChannel) -> QueryResult<Channel> {
        channels::table
            .filter(channels::link.eq(channel.link))
            .first::<Channel>(conn)
            .optional()
            .and_then(|r| match r {
                Some(r) => Ok(r),
                None => create(conn, channel)
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
    use ::models::*;

    pub fn insert_if_new(conn: &PgConnection, item: &NewItem) -> QueryResult<Option<Item>> {
        if item.guid.is_none() {
            Ok(Some(create(conn, item)?))
        } else {
            items::table
                .filter(items::guid.eq(item.guid))
                .first::<Item>(conn)
                .optional()
                .and_then(|r| match r {
                    Some(_) => Ok(None),
                    None => {
                        Ok(Some(create(conn, item)?))
                    }
                })
        }
    }

    pub fn create(conn: &PgConnection, item: &NewItem) -> QueryResult<Item> {
        insert_into(items::table)
            .values(item)
            .get_result(conn)
    }
}
