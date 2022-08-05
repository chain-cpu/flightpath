#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    #[test]
    fn test_success() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client
            .post("/flightpath/")
            .header(ContentType::JSON)
            .body(
                r##"{
                    "tickets": [
                        {
                            "src":"IND",
                            "dst":"EWR"
                        },
                        {
                            "src":"SFO",
                            "dst":"ATL"
                        },
                        {
                            "src":"GSO",
                            "dst":"IND"
                        },
                        {
                            "src":"ATL",
                            "dst":"GSO"
                        }
                    ]
                }"##,
            )
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        // assert_eq!(response.content_type(), Some(ContentType::JSON));
        let response_body = response.into_string().unwrap();
        let path: Ticket = serde_json::from_str(&response_body).expect("Valid User Response");
        assert_eq!(path.src, r"SFO");
        assert_eq!(path.dst, r"EWR");
    }

    #[test]
    fn test_failure() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client
            .post("/flightpath/")
            .header(ContentType::JSON)
            .body(
                r##"{
                    "tickets": [
                        {
                            "src":"ABC",
                            "dst":"DEF"
                        },
                        {
                            "src":"DEF",
                            "dst":"GCD"
                        },
                        {
                            "src":"LCD",
                            "dst":"GCD"
                        }
                    ]
                }"##,
            )
            .dispatch();

        assert_eq!(response.status(), Status::BadRequest);
    }
}
