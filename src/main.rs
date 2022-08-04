#[macro_use]
extern crate rocket;

use fasthash::metro;
use rocket::http::uri::Path;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[post("/", data = "<ticket_list>")]
fn get_path(ticket_list: Json<TicketList>) -> String {
    match ticket_list.get_path() {
        Ok(path) => format!("final path is {}", path),
        Err(err) => format!("Invalid Path Error"),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/flightpath", routes![get_path])
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
                if start != "" {
                    return Err(PathError);
                }
                end = key;
            } else if value != 0 {
                return Err(PathError);
            }
        }
        Ok((Ticket {
            src: start,
            dst: end,
        }))
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
