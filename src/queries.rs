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
    use schema::items_categories;
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

    pub fn add_category(conn: &PgConnection, item_category: &NewItemCategory) -> QueryResult<()> {
        insert_into(items_categories::table)
            .values(item_category)
            .execute(conn)
            .map(|_|())
    }
}

pub mod categories {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::QueryResult;
    use diesel::ExpressionMethods;
    use diesel::insert_into;
    use schema::categories;
    use ::models::*;

    pub fn get_or_create(conn: &PgConnection, category: &NewCategory) -> QueryResult<Category> {
        categories::table
            .filter(categories::channel_id.eq(category.channel_id).and(categories::name.eq(category.name)))
            .first::<Category>(conn)
            .optional()
            .and_then(|c| match c {
                Some(c) => Ok(c),
                None => {
                    create(conn, category)
                }
            })
    }

    pub fn create(conn: &PgConnection, cat: &NewCategory) -> QueryResult<Category> {
        insert_into(categories::table)
            .values(cat)
            .get_result(conn)
    }
}
