use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let connection_ip = "127.0.0.1";
    let connection_port = "7878";
    let listener = TcpListener::bind(connection_ip.to_owned() + ":" + connection_port).unwrap();
    println!("\nOpen connection on {}:{}", connection_ip, connection_port);

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