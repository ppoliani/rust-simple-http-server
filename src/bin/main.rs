use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use simple_http_server::request::thread_pool::ThreadPool;

fn handle_connection(mut stream:TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line, file) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
  };
  
  let content = fs::read_to_string(
    format!("src/assets/{}", file)
  ).unwrap();

  let response = format!("{}{}", status_line, content);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }

  println!("Shutting down.");
}
