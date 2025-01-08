use std::net::TcpListener;

use web_server::{handle_connection, multi_thread::ThreadPool};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    let thread_pool = ThreadPool::new(1);

    for connection in listener.incoming() {
        thread_pool.handle(|| {
            handle_connection(connection.unwrap());
        });
    }
}
