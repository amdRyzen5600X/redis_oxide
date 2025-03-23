use std::io::{Result, Write};

use crate::{
    Data, Value,
    handlers::{
        command_handlers::{decr, get, incr, set},
        start_handlers::handle_command_docs,
    },
    send_error,
};

pub fn route(req: Value, stream: &mut dyn Write, data: Data) -> Result<()> {
    match req {
        Value::Array(arr) => {
            if arr
                == vec![
                    Value::BulkString("COMMAND".to_string()),
                    Value::BulkString("DOCS".to_string()),
                ]
            {
                handle_command_docs(stream)?;
            } else if arr.starts_with(&[Value::BulkString("get".to_string())]) {
                if arr.len() != 2 {
                    send_error(stream, "Error, wrong amount of arguments for 'get' command")?;
                }
                let key = arr[1].to_string();
                get(data.clone(), &key, stream)?;
            } else if arr.starts_with(&[Value::BulkString("set".to_string())]) {
                if arr.len() != 3 {
                    send_error(stream, "Error, wrong amount of arguments for 'set' command")?;
                }
                let key = arr[1].to_string();
                let val = arr[2].clone();
                set(data.clone(), &key, val, stream)?;
            } else if arr.starts_with(&[Value::BulkString("incr".to_string())]) {
                if arr.len() != 2 {
                    send_error(
                        stream,
                        "Error, wrong amount of arguments for 'incr' command",
                    )?;
                }
                let key = arr[1].to_string();
                incr(data.clone(), &key, stream)?;
            } else if arr.starts_with(&[Value::BulkString("decr".to_string())]) {
                if arr.len() != 2 {
                    send_error(
                        stream,
                        "Error, wrong amount of arguments for 'decr' command",
                    )?;
                }
                let key = arr[1].to_string();
                decr(data.clone(), &key, stream)?;
            } else {
                send_error(stream, "ERR unknown command")?;
            }
        }
        _ => {
            send_error(stream, "ERR unknown command")?;
        }
    }

    Ok(())
}
