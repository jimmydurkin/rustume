#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::JsonValue;

mod config;
mod contact;
mod resume;
mod work;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "404 not found",
    })
}

fn main() {
    rocket::ignite()
        .mount(
            "/contact",
            routes![
                contact::name,
                contact::email,
                contact::phone,
                contact::location,
                contact::full
            ],
        )
        .mount("/work", routes![work::history, work::current])
        .mount("/", routes![resume::full])
        .register(catchers![not_found])
        .launch();
}
