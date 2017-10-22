extern crate grocerator;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    match (env::var("DATABASE_URL"), env::var("PRODUCTS_LIST_PATH")) {
        (Ok(connection_string), Ok(products_list_path)) => {
            grocerator::run(&connection_string, &products_list_path)
        }
        (Err(_), Ok(_)) => panic!("You need to provide DATABASE_URL environment variable!"),
        (Ok(_), Err(_)) => panic!("You need to provide PRODUCTS_LIST_PATH environment variable!"),
        _ => panic!("You need to provide both DATABASE_URL and PRODUCTS_LIST_PATH environment variables!"),
    };
}