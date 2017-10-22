#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod database;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn run(db_string: &str) {
    let pool = database::pool(db_string).expect("Connection pool creation failed");

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![index])
        .launch();
}

#[cfg(test)]
mod tests {}