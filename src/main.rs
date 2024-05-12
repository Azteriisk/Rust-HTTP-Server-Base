use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::env;

fn handle_client(mut stream: std::net::TcpStream) {
    let mut rdr = std::io::BufReader::new(&mut stream);
    let mut first_line = String::new();
    if let Err(err) = rdr.read_line(&mut first_line) {
        eprintln!("Error reading request line: {}", err);
        return;
    }
    let request_parts: Vec<&str> = first_line.trim().split(' ').collect();
    if request_parts.len() < 3 {
        eprintln!("Invalid request line: {}", first_line);
        return;
    }
    let method = request_parts[0];
    let resource = request_parts[1];
    let http_version = request_parts[2];
    if method != "GET" || http_version != "HTTP/1.1" {
        eprintln!("Unsupported method or HTTP version");
        return;
    }

    // Get the path to the directory containing the source file
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Construct the path to the htdocs directory relative to the source directory
    let mut htdocs_path = PathBuf::from(src_dir);
    htdocs_path.push("htdocs");

    // Construct the file path relative to the htdocs directory
    let mut file_path = PathBuf::from(&htdocs_path);
    file_path.push(resource.trim_start_matches("/"));
    if file_path.is_dir() {
        file_path.push("index.html");
    }

    let response = match std::fs::read_to_string(&file_path) {
        Ok(content) => format!("HTTP/1.1 200 OK\r\n\r\n{}", content),
        Err(err) => {
            eprintln!("Error reading file {:?}: {}", file_path, err);
            "HTTP/1.1 404 Not Found\r\n\r\nFile Not Found".to_string()
        }
    };
    if let Err(err) = stream.write_all(response.as_bytes()) {
        eprintln!("Error writing response: {}", err);
    }
}

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
            }
        }
    }
}