use std::{io::Result, net::TcpListener};

use redis_oxide::parse;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6969")?;
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let response = parse::parse(&mut stream);
            println!("{:?}", response);
        }
    }
    Ok(())
}
