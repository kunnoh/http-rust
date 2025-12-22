use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write, Result};
use log::{debug, error, info};

const SUCCESS_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const ERROR_RESPONSE: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    debug!("Accepted new connection: {}", stream.peer_addr()?);
    let mut request_buffer = BufReader::new(&*stream);
    let mut request_line = String::new();
    request_buffer.read_line(&mut request_line)?;

    // Split by space. Second field is path
    // Example: "GET /index.html HTTP/1.1"
    let path_vec: Vec<&str> = request_line.split_whitespace().collect();

    match &path_vec[..] {
        ["GET", path, "HTTP/1.1"] => {
            if *path == "/" {
                debug!("/ path requested");
                stream.write_all(SUCCESS_RESPONSE)?;
            } else {
                debug!("Unknown path {} requested", path);
                stream.write_all(ERROR_RESPONSE)?;
            }
        }
        _ => {
            error!("Invalid request: {}", request_line.trim());
            stream.write_all(ERROR_RESPONSE)?;
        }
    }

    stream.flush()
}
fn main() -> Result<()> {
    env_logger:: init();

    info!("Server starting!");
    let listener = TcpListener::bind("127.0.0.1:4221")?;
    info!("Server started at: http://{}", listener.local_addr()?);
    
    for tcp_stream in listener.incoming() {
        match tcp_stream {
            Ok(mut stream) => {
                if let Err(e) = handle_connection(&mut stream) {
                    error!("Error handle_connection: {e}");
                } 
            }
            Err(e) => error!("Connection failed:: {e}")
        }
    }

    Ok(())
}
