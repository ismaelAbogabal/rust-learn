use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    str::Chars,
};

use web_server::{handle_connection, request::HttpRequest};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    for connection in listener.incoming() {
        handle_connection(connection.unwrap());
    }
}
