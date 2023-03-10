use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2222").unwrap();
    let mut posts = String::new();

    println!("Server listening on port 2222");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut posts);
    }
}

fn handle_connection(mut stream: TcpStream, posts: &mut String) {
    let mut buffer = [0; 1024];
    _ = stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(post) {
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        let request = String::from_utf8_lossy(&buffer[..]);

        let post = parse_request_line(&request);
        posts.push_str(&post);
        posts.push_str("<br />");

        // send to database
        let mut db_stream = TcpStream::connect("localhost:5432").unwrap();
        db_stream.write_all(post.as_bytes()).unwrap();

        let mut result = [0; 1024];
        let table = match db_stream.read(&mut result) {
            Ok(_) => String::from_utf8_lossy(&result),
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
                return;
            }
        };

        let contents = format!(
            r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Hello!</title>
    </head>
    <body>
        {}
        <form method="post">
            <input type="text" name="sql" size="40">
            <input type="submit" value="send">
        </form>
        {}
    </body>
</html>"#,
            table, posts
        );

        let response = format!("{}{}", status_line, contents);

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        return;
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_request_line(request: &str) -> String {
    let mut post = String::new();

    for line in request.lines() {
        if let Some(stripped) = line.strip_prefix("sql=") {
            post += stripped;
        }
    }

    unescape(&post)
}

fn unescape(text: &str) -> String {
    text.replace('\0', "")
        .replace('+', " ")
        .replace("%2C", ",")
        .replace("%3B", ";")
        .replace("%28", "(")
        .replace("%29", ")")
        .replace("%27", "'")
}
