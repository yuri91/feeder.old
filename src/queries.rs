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
