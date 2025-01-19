use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

use rust_http_server::thread_pool_util::thread_pool::ThreadPool;

fn main() {
    let addr = "127.0.0.1:7878";

    let tcp_listener = TcpListener::bind(addr).unwrap();
    let thread_pool = ThreadPool::new(4);

    let (shut_sender, shut_receiver): (Sender<usize>, Receiver<usize>) = mpsc::channel();
    let arc_sender = Arc::new(shut_sender);

    for r_tcp_stream in tcp_listener.incoming() {
        let tcp_stream = r_tcp_stream.unwrap();
        let arc_sender_clone = Arc::clone(&arc_sender);
        thread_pool.execute(|| handle_connect(tcp_stream, arc_sender_clone));
        if shut_receiver.try_recv().is_ok() {
            break;
        }
    }

    println!("Shutdown start!");
    drop(thread_pool);
}

fn handle_connect(mut stream: TcpStream, shut_sender: Arc<Sender<usize>>) {
    let buf_reader = BufReader::new(&stream);
    let next = buf_reader.lines().next().unwrap().unwrap();
    let (response_code, response_file_path) = match next.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "html/hello.html")
        }
        "GET /shutdown HTTP/1.1" => ("SHUTDOWN", ""),
        _ => ("HTTP/1.1 404 NOT FOUND", "html/404.html"),
    };

    if response_code == "SHUTDOWN" {
        shut_sender.send(1).unwrap();
        return;
    }

    // file read
    let contents = fs::read_to_string(response_file_path).unwrap();
    let len = contents.len();

    // assemble returned string
    let return_obj = format!("{response_code}\r\nContent-Length:{len}\r\n\r\n{contents}");
    stream.write_all(return_obj.as_bytes()).unwrap();
}
