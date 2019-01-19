// (c) 2019 Joost Yervante Damad <joost@damad.be>

#[macro_use]
extern crate diesel;

pub mod db;
pub mod schema;
pub mod model;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> model::Post {
    use self::schema::posts;

    let new_post = model::insert::NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
