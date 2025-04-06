use std::io::{Result, Write};

use crate::{Data, Value, send_error};

pub fn get(data: Data, key: &str, stream: &mut dyn Write) -> Result<()> {
    let lock = data.lock().unwrap();
    let value = lock.get(key);
    if let Some(v) = value {
        stream.write_all(&v.to_bytes())?;
        stream.flush()?;
    } else {
        let _ = stream.write_all(&Value::Null(()).to_bytes());
        stream.flush()?;
    }
    Ok(())
}
pub fn set(data: Data, key: &str, value: Value, stream: &mut dyn Write) -> Result<()> {
    let mut lock = data.lock().unwrap();
    lock.insert(key.to_string(), value);
    let resp = Value::String("OK".to_string()).to_bytes();
    stream.write_all(&resp)?;
    stream.flush()?;
    Ok(())
}
pub fn incr(data: Data, key: &str, stream: &mut dyn Write) -> Result<()> {
    let mut lock = data.lock().unwrap();
    if let Some(v) = lock.get(key) {
        let v = v.to_string().parse::<i64>();
        if let Ok(v) = v {
            let v = Value::Integer(v + 1);
            stream.write_all(&v.to_bytes())?;
            lock.insert(key.to_string(), Value::BulkString(v.to_string()));
            stream.flush()?;
        } else {
            send_error(stream, "ERR value is not an integer or out of range")?;
        }
    } else {
        let v = Value::Integer(1);
        stream.write_all(&v.to_bytes())?;
        lock.insert(key.to_string(), Value::BulkString(v.to_string()));
        stream.flush()?;
    }
    Ok(())
}

pub fn decr(data: Data, key: &str, stream: &mut dyn Write) -> Result<()> {
    let mut lock = data.lock().unwrap();
    let value = lock.get(key);
    if let Some(v) = value {
        let v = v.to_string().parse::<i64>();
        if let Ok(v) = v {
            let v = Value::Integer(v - 1);
            stream.write_all(&v.to_bytes())?;
            lock.insert(key.to_string(), Value::BulkString(v.to_string()));
            stream.flush()?;
        } else {
            let resp =
                Value::Error("ERR value is not an integer or out of range".to_string()).to_bytes();
            stream.write_all(&resp)?;
            stream.flush()?;
        }
    } else {
        let v = Value::Integer(-1);
        stream.write_all(&v.to_bytes())?;
        lock.insert(key.to_string(), Value::BulkString(v.to_string()));
        stream.flush()?;
    }
    Ok(())
}

pub fn del(
    data: Data,
    keys: &mut impl Iterator<Item = String>,
    stream: &mut dyn Write,
) -> Result<()> {
    let mut count = 0;
    let mut lock = data.lock().unwrap();
    for key in keys {
        if lock.remove(&key).is_some() {
            count += 1;
        }
    }
    stream.write_all(&Value::Integer(count).to_bytes())
}
