use std::{io::{Read, Result}, net::TcpListener};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6969")?;
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buf = String::new();
            let _ = stream.read_to_string(&mut buf);
            println!("{}", buf);
        }
    }
    Ok(())
}
