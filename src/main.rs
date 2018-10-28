#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;


mod contact {

    use rocket_contrib::Json;

    #[derive(Serialize)]
    struct ContactFull {
        name: Name,
        email: String,
        phone: String,
        location: String,
    }

    #[derive(Serialize)]
    pub struct Name {
        first_name: String,
        last_name: String,
    }

    #[get("/name")]
    pub fn name() -> Json<Name> {
        Json(Name {
            first_name: "Jimmy".to_string(),
            last_name: "Durkin".to_string(),
        })
    }
    #[get("/phone")]
    pub fn phone() -> &'static str {
        "847-772-1466"
    }
    #[get("/email")]
    pub fn email() -> &'static str {
        "jdurkin@buio.ch"
    }
    #[get("/location")]
    pub fn location() -> &'static str {
        "Chicago, IL"
    }
}

fn main() {
    rocket::ignite()
        .mount("/contact", routes![contact::name,contact::email,contact::phone,contact::location])
        .launch();
}