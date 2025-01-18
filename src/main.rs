use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main() {
    let addr = "127.0.0.1:7878";

    let tcp_listener = TcpListener::bind(addr).unwrap();

    let x = tcp_listener.incoming();
    for y in x {
        let tcp_stream = y.unwrap();
        handle_connect(tcp_stream);
    }
}

fn handle_connect(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let next = buf_reader.lines().next().unwrap().unwrap();
    let (response_code, response_file_path) = match next.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html/hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "html/404.html")
    };

    // file read
    let contents = fs::read_to_string(response_file_path).unwrap();
    let len = contents.len();

    // assemble returned string
    let return_obj = format!("{response_code}\r\nContent-Length:{len}\r\n\r\n{contents}");
    stream.write_all(return_obj.as_bytes()).unwrap();
}
