// (c) 2019 Joost Yervante Damad <joost@damad.be>

use crate::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}