use log::{debug, error, info, warn};
use std::io::{BufRead, BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};

const SUCCESS_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const ERROR_RESPONSE: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    debug!("Accepted new connection: {}", stream.peer_addr()?);

    let mut request_buffer: BufReader<&TcpStream> = BufReader::new(&*stream);
    let mut request_line: String = String::new();
    request_buffer.read_line(&mut request_line)?;

    let path_vec: Vec<&str> = request_line.split_whitespace().collect();
    // println!("path: \n\t{:?}", path_vec);

    match &path_vec[..] {
        ["GET", path, "HTTP/1.1"] => {
            if *path == "/" {
                info!("GET - {path}");
                stream.write(SUCCESS_RESPONSE)?;
            } else {
                warn!("GET - {path}");
                stream.write(ERROR_RESPONSE)?;
            }
        }
        _ => {
            error!("Invalid request: {}", request_line.trim());
            stream.write(ERROR_RESPONSE)?;
        }
    }

    stream.flush()
}

fn main() -> Result<()> {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:4221")?;
    info!("Server started at: http://{}", listener.local_addr()?);

    for tcp_stream in listener.incoming() {
        match tcp_stream {
            Ok(mut stream) => {
                if let Err(e) = handle_connection(&mut stream) {
                    error!("Error handle_connection:\n\t {e}");
                }
            }
            Err(e) => error!("Connection failed:\n\t {e}"),
        }
    }

    Ok(())
}
