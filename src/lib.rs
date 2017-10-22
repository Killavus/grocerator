#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod database;
mod products;

use products::Product;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub struct AppState {
    pub db_pool: database::DbConnectionPool,
    pub product_list: Vec<Product>,
}

pub fn run(db_string: &str, products_file: &str) {
    let db_pool = database::pool(db_string).expect("Connection pool creation failed");
    let product_list = products::load(products_file).expect("Product list load failed");

    rocket::ignite()
        .manage(AppState {
                    db_pool,
                    product_list,
                })
        .mount("/", routes![index])
        .launch();
}

#[cfg(test)]
mod tests {}