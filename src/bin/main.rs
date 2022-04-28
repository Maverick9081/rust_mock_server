use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            connection_handler(stream);
        });
    }
}

fn connection_handler(mut stream:TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";


    let(status_line, file_name) =if buffer.starts_with(get){
        ("HTTP/1.1 200 OK","hello.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK","hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };
        
    let content = fs::read_to_string(file_name).unwrap();

        
    let response = format!(
        "{}\r\nContent-Length :{}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}