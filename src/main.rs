use rayon::spawn;
use std::{
    collections::HashMap,
    io::Result,
    net::TcpListener,
    sync::{Arc, Mutex},
};

use redis_oxide::{Data, parse, router::route};

fn main() -> Result<()> {
    let data: Data = Arc::new(Mutex::new(HashMap::new()));
    //let mut lock = data.lock().unwrap();
    //lock.insert("hello".to_string(), redis_oxide::Value::String("world".to_string()));
    let listener = TcpListener::bind("127.0.0.1:6969")?;
    for stream in listener.incoming() {
        let data = data.clone();
        spawn(move || {
            println!(".");
            if let Ok(mut stream) = stream {
                loop {
                    let mut buf = [0];
                    if let Ok(by) = stream.peek(&mut buf) {
                        if by == 0 {
                            break;
                        }
                    }
                    let request = parse::parse(&mut stream);
                    println!("{:?}", request);
                    if let Ok(req) = request {
                        let _ = route(req, &mut stream, data.clone());
                    }
                }
            }
            println!("connection is about to be lost")
        });
    }
    Ok(())
}
