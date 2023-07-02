use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
    ,fs};
use server::ThreadPool;
fn main() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("{:?}", listener);
    for stream in listener.incoming().take(100) {
        let stream = stream.unwrap();
        pool.execute(||{handle_connection(stream)});
    }
    drop(pool);

}



    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
            // println!("{:?}",http_request);
            if http_request.len() > 0 {
                println!("{}", http_request[0]);
                let universal_reqest: Vec<&str> = http_request[0].split(" ").collect();
                let universal_paths: Vec<&str>  = universal_reqest[1].split("/").collect();
                let universal_path = universal_paths[universal_paths.len()-1];
                println!("{}", universal_path);
                let (status_line, path) = if http_request[0]=="GET /css/style.css HTTP/1.1" {
                    ("HTTP/1.1 200 OK","server/css/style.css")
                } else if http_request[0]=="GET / HTTP/1.1" {
                    // thread::sleep(Duration::from_millis(100));
                    ("HTTP/1.1 200 OK","server/balls.html")
                } else if http_request[0]=="GET /main.js HTTP/1.1" {
                    ("HTTP/1.1 200 OK","server/main.js")
                } else {
                    ("HTTP/1.1 200 OK", universal_path)
                };
                let error404 = fs::read_to_string("server/404.html").unwrap();
                let content = match fs::read_to_string(path) {
                    Ok(o) => o,
                    Err(_) => error404
                };
                let len_of_content = content.len();
                let response = format!("{status_line}\r\nContent-Length: {len_of_content}\r\n\r\n{content}");
                stream.write_all(response.as_bytes()).unwrap();
                // println!("dcvdf");
            } 
    }       
