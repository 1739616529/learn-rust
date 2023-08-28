use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use learn_rust::ThreadPool;
fn main() {
    let proxy_server = TcpListener::bind("127.0.0.1:5551").unwrap();

    let pool = ThreadPool::bind(4);

    for stream in proxy_server.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })

    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n ";

    stream.write_all(response.as_bytes()).unwrap();
    // stream.all

    println!("Request: {:?}", http_request);
}
