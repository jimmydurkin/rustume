use config;
use contact;
use rocket_contrib::json::Json;
use work;

#[derive(Serialize)]
pub struct ResumeFull {
    contact: contact::ContactFull,
    work_history: Vec<work::WorkPlace>,
}

#[get("/full")]
pub fn full() -> Json<ResumeFull> {
    Json(ResumeFull {
        contact: contact::ContactFull {
            name: contact::Name {
                first_name: config::FIRST_NAME.to_string(),
                last_name: config::LAST_NAME.to_string(),
            },
            email: config::EMAIL.to_string(),
            phone: config::PHONE.to_string(),
            location: config::LOCATION.to_string(),
        },
        work_history: vec![work::parkwhiz(), work::uptake()],
    })
}
