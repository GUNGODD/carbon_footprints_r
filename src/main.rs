// Owenership

// lib
use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

// moduels/ paths
// #[path = "utils/calculator.rs"]
// mod calculator;
// #[path = "utils/project_carbon.rs"]
//mod project_carbon;
fn main() {
    server_testing();
}

fn server_testing() {
    // server tester
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Listening on http://127.0.0.1:7878");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}

// Clone and copy

//Implementaion in Rust
