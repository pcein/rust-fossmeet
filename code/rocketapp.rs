#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/<name>")]
fn index(name: &str) -> String {
    format!("\nHello, {}!, hope you are enjoying the Rust workshop!\n", name)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

