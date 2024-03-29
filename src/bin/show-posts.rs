extern crate diesel;
extern crate hello_rocket;

use self::hello_rocket::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use hello_rocket::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}