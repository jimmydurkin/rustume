#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod config {
    pub const FIRST_NAME: &str = "Jimmy";
    pub const LAST_NAME: &str = "Durkin";
    pub const EMAIL: &str = "jdurkin@buio.ch";
    pub const PHONE: &str = "847-772-1466";
    pub const LOCATION: &str = "Chicago, IL";

}

mod contact {
    use config;
    use rocket_contrib::Json;

    #[derive(Serialize)]
    pub struct ContactFull {
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
            first_name: config::FIRST_NAME.to_string(),
            last_name: config::LAST_NAME.to_string(),
        })
    }

    #[get("/phone")]
    pub fn phone() -> Json<> {
        Json(json!({
        "phone": config::PHONE
        }))
    }

    #[get("/email")]
    pub fn email() -> Json<> {
        Json(json!({
        "email": config::EMAIL
        }))
    }
    #[get("/location")]
    pub fn location() -> Json<> {
        Json(json!({
        "location": config::LOCATION
        }))
    }
    #[get("/full")]
    pub fn full() -> Json<ContactFull> {
        Json( ContactFull{
            name: Name { first_name: config::FIRST_NAME.to_string(), last_name: config::LAST_NAME.to_string(), },
            email: config::EMAIL.to_string(),
            phone: config::PHONE.to_string(),
            location: config::LOCATION.to_string(),
        })
    }
}

mod work {
    use rocket_contrib::Json;

    #[derive(Serialize)]
    pub struct WorkHistory {
        work_history: Vec<WorkPlace>,
    }

    #[derive(Serialize)]
    pub struct WorkPlace {
        name: String,
        title: String,
        dates: String,
        responsibilities: Vec<String>,
        achievements: Vec<String>,
    }

    fn uptake() -> WorkPlace {
        WorkPlace {
            name: "Uptake".to_string(),
            title: "Software Engineer".to_string(),
            dates: "May 2016 to Jan 2018".to_string(),
            responsibilities: vec![
                "Member of build-and-deploy team tasked with developing internal developer tooling.".to_string(),
                "Using gradle plugins and a custom ruby deploy script, decrease amount of work required \
                 by developers to deploy microservices.".to_string(),
                "Support and develop infrastructure necessary for development.(Jenkins, Artifactory, \
                 Bitbucket).".to_string(),
            ],
            achievements: vec![
                "Reduced spending on build fleet by implementing Kotlin microservice which scales fleet \
                based on real-time metrics of our Jenkins installation.".to_string(),
                "Became true 'devops engineer' by developing, monitoring, supporting, and securing the services \
                our team supplied to the engineering organization.".to_string()
            ]
        }
    }

    fn parkwhiz() -> WorkPlace {
        WorkPlace {
            name: "Parkwhiz".to_string(),
            title: "Devops Engineer".to_string(),
            dates: "Jan 2018 to Present".to_string(),
            responsibilities: vec![
                "Cloud computing architecture.".to_string(),
                "Deployment and orchestration of dockerized microservices using chef and bash scripts".to_string(),
                "Monitor and enforce AWS security best practices.".to_string(),
                "Improve on-call experience by introducing fault-tolerance at the infrastructure level.".to_string(),
                "Deprecate and migrate infra on Linode and Digital Ocean to AWS.".to_string(),
                "Advise developers on architecture and security policy".to_string(),

            ],
            achievements: vec![
                "Implemented sensible network architecture and documentation allowing deprecation of legacy systems.".to_string(),
                "Automated the provisioning of dev and staging environment allowing the company to increase \
                development throughput and enable partner integrations.".to_string(),
                "Learned a ton about cloud networking and greatly improved my sysadmin skills and grown \
                my love for infra-network-security automation".to_string(),
            ]
        }
    }


    #[get("/history")]
    pub fn history() -> Json<WorkHistory> {
        Json(WorkHistory {
            work_history: vec![parkwhiz(), uptake()]
        })
    }
}

fn main() {

    rocket::ignite()
        .mount("/contact",
               routes![contact::name,contact::email,contact::phone,contact::location,contact::full])
        .mount("/work", routes![work::history])
        .launch();
}