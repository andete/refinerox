// (c) 2019 Joost Yervante Damad <joost@damad.be>

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
}