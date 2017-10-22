#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn run() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[cfg(test)]
mod tests {}