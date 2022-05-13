use std::io;
use std::net::TcpListener;

use http_server;

const PORT: &str = "5050";
const SERVER_ADDR: &str = "127.0.0.1";

extern crate chrono;
extern crate bufstream;

fn main() -> io::Result<()> {

    let server_conn = format!("{0}:{1}",SERVER_ADDR,PORT);

    println!("Initializing server");

    let socket_listener = TcpListener::bind(server_conn)?;

    println!("Server listening on {}",PORT);

    for stream in socket_listener.incoming() {
        match stream {
            Ok(stream) => {
                match http_server::handle_client(stream) {
                    Err(e) => println!("Error handling client: {}", e),
                    _ => (),
                }
            },
            Err(e) => println!("Connection failed: {}",e)
        }
    };

    Ok(())
}
