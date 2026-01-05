use log::{debug, error, info, warn};
use std::io::{BufRead, BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};

// const SUCCESS_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const ERROR_RESPONSE: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

type Key = String;
type Value = String;

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    debug!("Accepted new connection: {}", stream.peer_addr()?);

    let mut request_buffer: BufReader<&TcpStream> = BufReader::new(&stream);
    let mut request_line: String = String::new();
    request_buffer.read_line(&mut request_line)?;

    let mut headers: Vec<(Key, Value)> = Vec::new();
    loop {
        let mut header_line: String = String::new();
        let next_header: usize = request_buffer.read_line(&mut header_line)?;
        if header_line == "\r\n" || next_header == 0 {
            break;
        }
        let Some((key, value)) = header_line.split_once(": ") else {
            error!("Invalid header line in request:\t{header_line}");
            continue;
        };
        headers.push((key.to_string(), value.trim_end().to_string()));
    }
    // debug!("{:?}", headers);

    let path_vec: Vec<&str> = request_line.split_whitespace().collect();
    // println!("path: \n\t{:?}", path_vec);

    match &path_vec[..] {
        ["GET", path, "HTTP/1.1"] => {
            info!("GET {path}");
            if *path == "/" {
                let body: String = format!("Welcome!\nPath {}", path);
                let response: String = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write(response.as_bytes());
                stream.flush()?;
            } else if path.starts_with("/user-agent") {
                let mut user_agent = None;
                for (key, value) in headers {
                    if key == "User-Agent" {
                        user_agent = Some(value);
                        break;
                    }
                }

                match user_agent {
                    None => {
                        error!("User-agent header not found!");
                        let _ = stream.write(ERROR_RESPONSE)?;
                        stream.flush()?;
                    }
                    Some(user_agent) => {
                        let body: String = format!("Welcome!\nUser agent: {user_agent}");
                        let response: String = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                            body.len(),
                            body
                        );
                        let _ = stream.write(response.as_bytes())?;
                        stream.flush()?;
                    }
                }
            } else if path.starts_with("/echo") {
                let echo_path: Option<(&str, &str)> = path.split_once("/echo");
                match echo_path {
                    Some((_, path)) => {
                        let body: String = format!("Welcome!\nPath /echo{}", path);
                        let response: String = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                            body.len(),
                            body
                        );

                        let _ = stream.write(response.as_bytes());
                        stream.flush()?;
                    }
                    _ => {
                        error!("Invalid path: {path}");
                        let _ = stream.write(ERROR_RESPONSE);
                        stream.flush()?;
                    }
                }
            } else {
                warn!("GET {path}");
                let body: String = format!("{path} not found");
                let response: String = format!(
                    "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write(response.as_bytes());
                stream.flush()?;
            }
        }
        _ => {
            error!("Invalid request: {}", request_line.trim());
            let _ = stream.write(ERROR_RESPONSE);
            stream.flush()?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    //Enable logging
    env_logger::init();

    //Start server
    let listener = TcpListener::bind("0.0.0.0:4221")?;
    info!("Server started at: http://{}", listener.local_addr()?);

    for tcp_stream in listener.incoming() {
        match tcp_stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    error!("Error handle_connection:\n\t{e}");
                }
            }
            Err(e) => error!("Connection failed:\n\t{e}"),
        }
    }

    Ok(())
}
