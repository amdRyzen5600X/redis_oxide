use std::io::{Result, Write};

use crate::{
    Data, Value,
    handlers::{
        command_handlers::{decr, del, get, incr, set},
        start_handlers::handle_command_docs,
    },
    send_error,
};

macro_rules! handle {
    //command: "command name", arguments for command
    {$($data:ident)?, $stream:ident, $arr:ident, $command:ident, $key:tt $(,$arg:tt)*} => {
        let $key = $arr.next();
        let Some(crate::Value::BulkString($key)) = $key else {
            return crate::send_error(
                $stream,
                &format!(
                    "Error, wrong amount of arguments for '{}' command",
                    stringify!($command)
                ),
            );
        };
        $(
        let $arg = $arr.next();
        let Some($arg) = $arg else {
            return crate::send_error(
                $stream,
                &format!(
                    "Error, wrong amount of arguments for '{}' command",
                    stringify!($command)
                ),
            );
        };
        )*
        return $command($($data.clone(),)? $key, $($arg.clone(),)* $stream);
    };
    {$data:ident, $stream:ident, $arr:ident, $command:ident} => {
                    let mut keys = $arr.map(|v| v.to_string());
                    if keys.len() == 0 {
                        return crate::send_error(
                            $stream,
                            &format!(
                                "Error, wrong amount of arguments for '{}' command",
                                stringify!($command)
                            ),
                        );
                    }
                    $command($data.clone(), &mut keys, $stream)
    };
}

pub fn route(req: Value, stream: &mut dyn Write, data: Data) -> Result<()> {
    match req {
        Value::Array(arr) => {
            let mut arr = arr.iter();
            let command = match arr.next() {
                Some(command) => command,
                None => return Ok(()),
            };
            match command {
                Value::BulkString(cmd) if cmd.to_lowercase() == "command" => {
                    handle! {, stream, arr, handle_command_docs, arg}
                }
                Value::BulkString(cmd) if cmd.to_lowercase() == "get" => {
                    handle! {data, stream, arr, get, key}
                }
                Value::BulkString(cmd) if cmd.to_lowercase() == "set" => {
                    handle! {data, stream, arr, set, key, val}
                }
                Value::BulkString(cmd) if cmd.to_lowercase() == "incr" => {
                    handle! {data, stream, arr, incr, key}
                }
                Value::BulkString(cmd) if cmd.to_lowercase() == "decr" => {
                    handle! {data, stream, arr, decr, key}
                }
                Value::BulkString(cmd) if cmd.to_lowercase() == "del" => {
                    handle! {data, stream, arr, del}
                }
                _ => send_error(stream, "ERR unknown command"),
            }
        }
        _ => send_error(stream, "ERR unknown command"),
    }
}
