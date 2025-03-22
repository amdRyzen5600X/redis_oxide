use std::{
    collections::HashMap,
    io::Result,
    net::TcpListener,
    sync::{Arc, Mutex},
};

use redis_oxide::{
    Data,
    parse::{self},
    router::route,
};

fn main() -> Result<()> {
    let data: Data = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:6969")?;
    for stream in listener.incoming() {
        println!(".");
        if let Ok(mut stream) = stream {
            loop {
                let mut buf = [0];
                let by = stream.peek(&mut buf);
                if let Ok(by) = by {
                    if by == 0 {
                        break;
                    }
                }
                let request = parse::parse(&mut stream);
                println!("{:?}", request);
                if let Ok(req) = request {
                    route(req, &mut stream, data.clone())?;
                }
            }
        }
        println!("connection is about to be lost")
    }
    Ok(())
}
