use request::HttpRequest;
use response::Response;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufReader, Write},
    net::TcpStream,
};
use uuid::Uuid;

pub mod request;
pub mod response;

pub fn handle_connection(mut conn: TcpStream) {
    let request = HttpRequest::new(BufReader::new(&conn));

    let results: String;
    let status: usize;
    let status_message: String;

    let mut headers: String = String::new();

    if request.endpoint == "/" {
        let response = Response::html_file(200, "home.html");
        conn.write(&response.to_string().into_bytes()).unwrap();
    }
    if request.endpoint == "/json" {
        let my_uuid = Uuid::new_v4();
        let json = DumbResponse {
            uuid: my_uuid.to_string(),
        };

        let response = Response::json(200, json);

        conn.write(&response.to_string().as_bytes()).unwrap();
    } else {
        results = "Page not found".to_string();
        status = 404;
        status_message = "NOT_FOUND".to_string();
        headers += &format!("Content-Length: {}\r\n", results.len());
        let response = format!("HTTP/1.1 {status} {status_message}\r\n{headers}\r\n{results}");
        conn.write(&response.into_bytes()).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DumbResponse {
    uuid: String,
}
