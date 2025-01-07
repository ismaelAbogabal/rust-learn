use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub struct HttpRequest {
    pub version: String,
    pub method: String,
    pub endpoint: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn new(reader: BufReader<&TcpStream>) -> Self {
        let mut lines = reader.lines();

        let first_line = lines.next().unwrap().unwrap();

        let mut splitted = first_line.split(" ");

        let method = splitted.next().unwrap().to_string();
        let endpoint = splitted.next().unwrap().to_string();
        let version = splitted.next().unwrap().to_string();

        let mut headers: HashMap<String, String> = HashMap::new();
        while let Some(line) = lines.next() {
            let line = line.unwrap();

            if line == "" {
                break;
            }

            let mut split = line.split(":");

            match (split.next(), split.next()) {
                (Some(k), Some(v)) => {
                    headers.insert(k.to_string(), v.to_string());
                }
                _ => {}
            }
        }

        let mut body = String::new();
        if method != "GET" && method != "OPTIONS" && method != "OPTIONS" && method != "OPTIONS" {
            while let Some(line) = lines.next() {
                let line = line.unwrap();

                if line == "" {
                    break;
                }
                println!("{line}");
                body.push_str(&line);
            }
        }

        HttpRequest {
            version,
            endpoint,
            method,
            headers,
            body,
        }
    }
}
