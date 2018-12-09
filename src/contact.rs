use config;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize)]
pub struct ContactFull {
    pub name: Name,
    pub email: String,
    pub phone: String,
    pub location: String,
}

#[derive(Serialize)]
pub struct Name {
    pub first_name: String,
    pub last_name: String,
}

#[get("/name")]
pub fn name() -> Json<Name> {
    Json(Name {
        first_name: config::FIRST_NAME.to_string(),
        last_name: config::LAST_NAME.to_string(),
    })
}

#[get("/phone")]
pub fn phone() -> JsonValue {
    json!({ "phone": config::PHONE })
}

#[get("/email")]
pub fn email() -> JsonValue {
    json!({ "email": config::EMAIL })
}
#[get("/location")]
pub fn location() -> JsonValue {
    json!({ "location": config::LOCATION })
}
#[get("/full")]
pub fn full() -> Json<ContactFull> {
    Json(ContactFull {
        name: Name {
            first_name: config::FIRST_NAME.to_string(),
            last_name: config::LAST_NAME.to_string(),
        },
        email: config::EMAIL.to_string(),
        phone: config::PHONE.to_string(),
        location: config::LOCATION.to_string(),
    })
}
