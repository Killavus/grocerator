extern crate grocerator;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    match env::var("DATABASE_URL") {
        Ok(connection_string) => grocerator::run(&connection_string),
        Err(_) => panic!("You need to provide DATABASE_URL environment variable!"),
    };
}