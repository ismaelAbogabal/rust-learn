use request::HttpRequest;
use response::Response;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Write};
use std::net::TcpStream;
use uuid::Uuid;

pub mod multi_thread;
pub mod request;
pub mod response;

pub fn handle_connection(mut conn: TcpStream) {
    let request = HttpRequest::new(BufReader::new(&conn));

    if request.endpoint == "/" {
        let response = Response::html_file(200, "home.html");
        conn.write(&response.to_string().into_bytes()).unwrap();
    } else if request.endpoint == "/json" {
        let json = DumbResponse {
            uuid: Uuid::new_v4().to_string(),
        };

        let response = Response::json(200, json);

        conn.write(&response.to_string().as_bytes()).unwrap();
    } else {
        let response = Response::text(404, "Not found".to_string());
        conn.write(&response.to_string().as_bytes()).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DumbResponse {
    uuid: String,
}
