use std::{
    collections::HashMap,
    fs::File,
    io::{Result, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
};

use redis_oxide::{
    Value,
    parse::{self, parse},
};

fn prepare_docs() -> Vec<u8> {
    let mut file = File::open("resp_docs.txt").expect("ERROR: cannot find file 'resp_docs.tst'");
    let resp = parse(&mut file).expect("ERROR: cannot parse prepared presponse");
    resp.to_bytes()
}

fn main() -> Result<()> {
    let data: Arc<Mutex<HashMap<String, Value>>> = Arc::new(Mutex::new(HashMap::new()));
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
                    match req {
                        Value::Array(arr) => {
                            if arr
                                == vec![
                                    Value::BulkString("COMMAND".to_string()),
                                    Value::BulkString("DOCS".to_string()),
                                ]
                            {
                                let resp = prepare_docs();
                                stream.write_all(&resp)?;
                                stream.flush()?;
                            } else if arr.starts_with(&[Value::BulkString("get".to_string())]) {
                                if arr.len() != 2 {
                                    let resp = Value::Error(
                                        "Error, wrong amount of arguments for 'get' command"
                                            .to_string(),
                                    )
                                    .to_bytes();
                                    stream.write_all(&resp)?;
                                    stream.flush()?;
                                    continue;
                                }
                                let key = arr[1].to_string();
                                let lock = data.lock().unwrap();
                                let value = lock.get(&key);
                                if let Some(v) = value {
                                    stream.write_all(&v.to_bytes())?;
                                    stream.flush()?;
                                } else {
                                    let _ = stream.write_all(&Value::Null(()).to_bytes());
                                    stream.flush()?;
                                }
                            } else if arr.starts_with(&[Value::BulkString("set".to_string())]) {
                                if arr.len() != 3 {
                                    let resp = Value::Error(
                                        "Error, wrong amount of arguments for 'set' command"
                                            .to_string(),
                                    )
                                    .to_bytes();
                                    stream.write_all(&resp)?;
                                    stream.flush()?;
                                    continue;
                                }
                                let key = arr[1].to_string();
                                let val = arr[2].clone();
                                let mut lock = data.lock().unwrap();
                                lock.insert(key, val);
                                let resp = Value::String("OK".to_string()).to_bytes();
                                stream.write_all(&resp)?;
                                stream.flush()?;
                            } else if arr.starts_with(&[Value::BulkString("incr".to_string())]) {
                                if arr.len() != 2 {
                                    let resp = Value::Error(
                                        "Error, wrong amount of arguments for 'incr' command"
                                            .to_string(),
                                    )
                                    .to_bytes();
                                    stream.write_all(&resp)?;
                                    stream.flush()?;
                                    continue;
                                }
                                let key = arr[1].to_string();
                                let mut lock = data.lock().unwrap();
                                let value = lock.get(&key);
                                if let Some(v) = value {
                                    let v = v.to_string().parse::<i64>();
                                    if let Ok(v) = v {
                                        let v = Value::Integer(v + 1);
                                        stream.write_all(&v.to_bytes())?;
                                        lock.insert(key, Value::BulkString(v.to_string()));
                                        stream.flush()?;
                                    } else {
                                        let resp = Value::Error(
                                            "ERR value is not an integer or out of range"
                                                .to_string(),
                                        )
                                        .to_bytes();
                                        stream.write_all(&resp)?;
                                        stream.flush()?;
                                        continue;
                                    }
                                } else {
                                    let v = Value::Integer(1);
                                    stream.write_all(&v.to_bytes())?;
                                    lock.insert(key, Value::BulkString(v.to_string()));
                                    stream.flush()?;
                                }
                            } else if arr.starts_with(&[Value::BulkString("decr".to_string())]) {
                                if arr.len() != 2 {
                                    let resp = Value::Error(
                                        "Error, wrong amount of arguments for 'decr' command"
                                            .to_string(),
                                    )
                                    .to_bytes();
                                    stream.write_all(&resp)?;
                                    stream.flush()?;
                                    continue;
                                }
                                let key = arr[1].to_string();
                                let mut lock = data.lock().unwrap();
                                let value = lock.get(&key);
                                if let Some(v) = value {
                                    let v = v.to_string().parse::<i64>();
                                    if let Ok(v) = v {
                                        let v = Value::Integer(v - 1);
                                        stream.write_all(&v.to_bytes())?;
                                        lock.insert(key, Value::BulkString(v.to_string()));
                                        stream.flush()?;
                                    } else {
                                        let resp = Value::Error(
                                            "ERR value is not an integer or out of range"
                                                .to_string(),
                                        )
                                        .to_bytes();
                                        stream.write_all(&resp)?;
                                        stream.flush()?;
                                        continue;
                                    }
                                } else {
                                    let v = Value::Integer(-1);
                                    stream.write_all(&v.to_bytes())?;
                                    lock.insert(key, Value::BulkString(v.to_string()));
                                    stream.flush()?;
                                }
                            } else {
                                let resp =
                                    Value::Error("ERR unknown command".to_string()).to_bytes();
                                stream.write_all(&resp)?;
                                stream.flush()?;
                                continue;
                            }
                        }
                        _ => {
                            let resp = Value::Error("ERR unknown command".to_string()).to_bytes();
                            stream.write_all(&resp)?;
                            stream.flush()?;
                            continue;
                        }
                    }
                }
            }
        }
        println!("connection is about to be lost")
    }
    Ok(())
}
