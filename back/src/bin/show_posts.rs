use refinerox_back::model::*;
use diesel::prelude::*;

fn main() {
    use refinerox_back::schema::posts::dsl::*;

    let connection = refinerox_back::db::establish_connection();
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}