use hello::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

/// Starts a TCP server on localhost:7878 to handle HTTP requests.
///
/// The server listens for incoming TCP connections on port 7878 of the local machine.
/// Upon receiving a connection, it utilizes a ThreadPool with 4 threads to handle the connections concurrently.
/// Each connection is processed by the `handle_connection` function.
///
/// # Panics
///
/// - Panics if the server fails to bind to the specified address.
/// - Panics if an incoming connection cannot be processed.
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

/// Processes an HTTP request and sends a response.
///
/// Reads the request line from the TCP stream to determine the requested path.
/// Based on the path, it selects an appropriate response status and content file.
/// - For the root path ("/"), it serves "hello.html".
/// - For "/sleep", it simulates a delay before serving "hello.html".
/// - For any other path, it responds with a 404 status and serves "404.html".
///
/// After determining the response, it reads the content file, constructs the HTTP response with
/// headers, and writes it back to the TCP stream.
///
/// # Panics
///
/// - Panics if reading from the stream or file fails.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}