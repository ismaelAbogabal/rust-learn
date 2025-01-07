use std::{collections::HashMap, fs, result};

use serde::Serialize;

#[derive(Debug)]
pub struct Response {
    pub status: usize,
    pub headers: Vec<String>,
    pub body: String,
}

impl Response {
    pub fn get_message(&self) -> String {
        match self.status {
            200 => String::from("OK"),
            400 => String::from("INVALID_REQUEST"),
            404 => String::from("Not_FOUND"),
            _ => String::from("Unkowen "),
        }
    }

    pub fn to_string(&self) -> String {
        //todo H

        let headers: String = self.headers.iter().map(|f| format!("{f}\r\n")).collect();

        println!("getting message");

        format!(
            "HTTP/1.1 {} {}\r\n{}\r\n{}",
            self.status,
            self.get_message(),
            headers,
            self.body
        )
    }
    pub fn text(status: usize, content: String) -> Self {
        let len = content.len();
        Self {
            body: content,
            headers: vec![format!("Content-Length: {len}")],
            status,
        }
    }

    pub fn html_file(status: usize, file: &str) -> Self {
        let file = fs::read_to_string(file).unwrap();

        Self {
            headers: vec![
                format!("Content-Length: {}", file.len()),
                String::from("Content-Type: text/html"),
            ],
            body: file,
            status,
        }
    }

    pub fn json<T: Serialize>(status: usize, json: T) -> Self {
        let val = serde_json::to_string(&json).unwrap();

        Self {
            headers: vec![
                format!("Content-Length: {}", val.len()),
                String::from("Content-Type: application/json"),
            ],
            body: val,
            status,
        }
    }
}
