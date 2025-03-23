use std::{
    fs::File,
    io::{Result, Write},
};

use crate::parse::parse;

fn prepare_docs() -> Vec<u8> {
    let mut file = File::open("resp_docs.txt").expect("ERROR: cannot find file 'resp_docs.tst'");
    let resp = parse(&mut file).expect("ERROR: cannot parse prepared presponse");
    resp.to_bytes()
}

pub fn handle_command_docs(arg: &str, stream: &mut dyn Write) -> Result<()> {
    if arg.to_lowercase() == "docs" {
        let resp = prepare_docs();
        stream.write_all(&resp)?;
        stream.flush()?;
    }
    Ok(())
}
