use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

use bufstream::BufStream;
use chrono::prelude::*;

struct Request {
    http_version: String,
    method: String,
    path: String,
    time: DateTime<Local>,
}

pub fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut buf = BufStream::new(stream);
    let mut request_line = String::new();

    //Get the first line of the request
    buf.read_line(&mut request_line)?;
    match parse_request(&mut request_line) {
        Ok(request) => {
            log_request(&request);
        },
        Err(()) => {
            println!("Bad request: {}", &request_line);
        },
    }
    Ok(())
}

fn parse_request(request: &mut String) -> Result<Request,()> {
    let mut string_parts = request.split(" ");
    let http_verb = match string_parts.next() {
        Some(http_verb) => http_verb.trim().to_string(),
        None => return Err(()),
    };
    let path = match string_parts.next() {
        Some(path) => path.trim().to_string(),
        None => return Err(()),
    };
    let http_version = match string_parts.next() {
        Some(http_version) => http_version.trim().to_string(),
        None => return Err(()),
    };
    let time = Local::now();
    Ok(Request {
        http_version,
        method: http_verb,
        path,
        time
    })
}

fn log_request(request: &Request) {
    println!("{0} {1} {2} {3}",request.time,request.method,request.path,request.http_version);
}