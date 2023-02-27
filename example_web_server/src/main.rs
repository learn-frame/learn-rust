use std::io::prelude::*; // 用于获取读写流所需的特定 trait
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};
use web_server::{PoolCreationError, ThreadPool};

/*
单线程 server 会依次处理每一个请求,
意味着它在完成第一个连接的处理之前不会处理第二个连接.
如果 server 正接收越来越多的请求, 这类串行操作会使性能越来越差
*/

fn main() {
    thread_poll();
}

#[allow(unused)]
fn single_thread_mode() {
    // bind 函数类似于 new 函数, 在这里它返回一个新的 TcpListener 实例
    // 在网络领域, 连接到监听端口被称为绑定到一个端口(binding to a port)
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(err) => panic!("{}", err),
    };

    // incoming 方法返回一个迭代器, 提供 stream 流
    // 这个遍历叫做连接尝试(connection attempts), 因为有很多原因就挂了
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);

        // println!("Connection established!");
    }
}

#[allow(unused)]
fn finite_number_of_multi_threads() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn thread_poll() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let poll = match ThreadPool::new(4) {
        Ok(thread_poll) => thread_poll,
        Err(e) => {
            let PoolCreationError { error_msg, code } = e;
            panic!("[{}] {}", code, error_msg);
        }
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        poll.execute(|| handle_connection(stream));
    }
}

// 一般读取是不可变的, 但 stream 是可变的, 这里得用 mut
fn handle_connection(mut stream: TcpStream) {
    // 创建了一个 1024 字节的缓冲区(数组), 元素缺省为 0
    let mut buffer = [0; 1024];

    // 把流的数据存放到缓冲区中
    stream.read(&mut buffer).unwrap();

    // 把缓冲区的数据卷成字符串, 用 from_utf8_lossy 是因为数据中可能有一些非 UTF-8 的序列
    // 这种被转换成 �
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get_html = b"GET / HTTP/1.1\r\n";
    let get_json = b"GET /api/posts HTTP/1.1\r\n";

    let (status_line, filename, content_type) = if buffer.starts_with(get_html) {
        ("HTTP/1.1 200 OK", "index.html", "text/html")
    } else if buffer.starts_with(get_json) {
        // 模拟单线程如果有一个耗时的, 其他的就慢了
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "data.json", "application/json")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html", "text/html")
    };

    let file = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
        status_line,
        file.len(),
        content_type,
        file
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
