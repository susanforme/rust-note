use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use server::thread_pool::ThreadPool;

fn main() {
    // 监听本机7878
    let listener = create_server(7878);
    let pool = ThreadPool::new(4);
    // 在优雅停机 server 之前只接受两个请求 .take(2)
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
fn create_server(port: usize) -> TcpListener {
    match TcpListener::bind(format!("127.0.0.1:{port}")) {
        Ok(l) => {
            println!("server is running at port {}", port);
            return l;
        }
        Err(_) => create_server(port + 1),
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // 根目录的请求
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        // 模拟慢请求
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let len = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    // let http_request: Vec<_> = buf_reader
    //     // lines 方法通过遇到换行符（newline）字节就切分数据流的方式返回一个 Result<String,std::io::Error> 的迭代器
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
}
