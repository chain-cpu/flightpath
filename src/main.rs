#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::serde::{
    json::{json, Json, Value},
    Deserialize, Serialize,
};
use std::collections::HashMap;
use std::fmt;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found",
    })
}

#[post("/", data = "<ticket_list>")]
fn get_path(ticket_list: Json<TicketList>) -> Result<Json<Ticket>, Status> {
    match ticket_list.get_path() {
        Ok(path) => return Ok(Json(path)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[launch]
fn rocket() -> _ {
    println!("FlightPath Microservice launched successfully.");
    rocket::build()
        .mount("/flightpath", routes![get_path])
        .register("/", catchers![not_found])
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct TicketList {
    tickets: Vec<Ticket>,
}
#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Ticket {
    #[field(validate = len(..30))]
    pub src: String,
    #[field(validate = len(..20))]
    pub dst: String,
}

impl TicketList {
    pub fn get_path(&self) -> Result<Ticket, PathError> {
        let iter = self.tickets.iter();
        let mut hash_map: HashMap<String, i32> = HashMap::new();
        for ticket in iter {
            println!("ticket: {}", ticket);
            *hash_map.entry(ticket.src.clone()).or_insert(0) += 1;
            *hash_map.entry(ticket.dst.clone()).or_insert(0) -= 1;
        }
        let mut start = "".to_string();
        let mut end = "".to_string();

        for (key, value) in hash_map {
            if value == 1 {
                if start != "" {
                    return Err(PathError);
                }
                start = key;
            } else if value == -1 {
                if end != "" {
                    return Err(PathError);
                }
                end = key;
            } else if value != 0 {
                return Err(PathError);
            }
        }
        Ok(Ticket {
            src: start,
            dst: end,
        })
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "src: {}, dst: {}", self.src, self.dst)
    }
}
#[derive(Debug, Clone)]
struct PathError;

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid ticket list")
    }
}
