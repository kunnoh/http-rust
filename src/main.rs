use std::net::TcpListener;

use log::{debug, error, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger:: init();

    info!("Server starting!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    info!("Server listening on: http://{}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                debug!("Accepted new connection:: {}", stream.peer_addr()?);
            }
            Err(e) => error!("Err: \n{e}")
        }
    }
    Ok(())
}
