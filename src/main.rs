use std::io::{BufRead, Write}; // Import necessary traits for I/O operations
use std::path::{PathBuf}; // Import necessary type for path manipulation
use std::env; // Import environment module for getting current directory

fn handle_client(mut stream: std::net::TcpStream) {
    let mut rdr = std::io::BufReader::new(&mut stream); // Create a buffered reader for the incoming stream
    let mut first_line = String::new(); // Create a mutable string to store the first line of the HTTP request
    if let Err(err) = rdr.read_line(&mut first_line) { // Read the first line of the HTTP request
        eprintln!("Error reading request line: {}", err); // Print an error message if reading fails
        return; // Exit the function if reading fails
    }
    let request_parts: Vec<&str> = first_line.trim().split(' ').collect(); // Split the request line into parts
    if request_parts.len() < 3 { // Check if the request line has at least three parts
        eprintln!("Invalid request line: {}", first_line); // Print an error message if the request line is invalid
        return; // Exit the function if the request line is invalid
    }
    let method = request_parts[0]; // Extract the HTTP method (e.g., GET, POST)
    let resource = request_parts[1]; // Extract the requested resource (e.g., /index.html)
    let http_version = request_parts[2]; // Extract the HTTP version (e.g., HTTP/1.1)
    if method != "GET" || http_version != "HTTP/1.1" { // Check if the method or HTTP version is unsupported
        eprintln!("Unsupported method or HTTP version"); // Print an error message if the method or HTTP version is unsupported
        return; // Exit the function if the method or HTTP version is unsupported
    }

    // Get the path to the directory containing the executable
    let exe_dir = env::current_dir().expect("Failed to get current directory");

    // Construct the path to the htdocs directory relative to the project root
    let htdocs_path = exe_dir.join("htdocs");

    // Construct the file path relative to the htdocs directory
    let mut file_path = PathBuf::from(&htdocs_path);
    file_path.push(resource.trim_start_matches("/"));
    if file_path.is_dir() {
        file_path.push("index.html");
    }

    let response = match std::fs::read_to_string(&file_path) {
        Ok(content) => format!("HTTP/1.1 200 OK\r\n\r\n{}", content), // If file is found, construct a success response
        Err(err) => {
            eprintln!("Error reading file {:?}: {}", file_path, err); // Print an error message if reading the file fails
            "HTTP/1.1 404 Not Found\r\n\r\nFile Not Found".to_string() // If file is not found, construct a "Not Found" response
        }
    };
    if let Err(err) = stream.write_all(response.as_bytes()) { // Write the response to the stream
        eprintln!("Error writing response: {}", err); // Print an error message if writing fails
    }
}

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap(); // Bind the TCP listener to the specified address
    for stream in listener.incoming() { // Iterate over incoming connections
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || { // Spawn a new thread to handle each connection
                    handle_client(stream); // Handle the client request
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err); // Print an error message if accepting the connection fails
            }
        }
    }
}
