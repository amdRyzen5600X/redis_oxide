use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
};

use crate::{MyFloat, ParseError, Value, VerbatimString};

pub fn parse(stream: &mut dyn Read) -> Result<Value, ParseError> {
    let mut buf = [1];
    let _ = stream.read(&mut buf);
    let datatype = buf[0] as char;
    match datatype {
        '+' => Ok(Value::String(parse_simple_string(stream)?)),
        '-' => Ok(Value::Error(parse_simple_string(stream)?)),
        ':' => Ok(Value::Integer(parse_integer(stream)?)),
        '$' => {
            let bstring = parse_bulk_string(stream)?;
            Ok(match bstring {
                Some(bstring) => Value::BulkString(bstring),
                None => Value::Null(()),
            })
        }
        '*' => {
            let arr = parse_array(stream)?;
            Ok(match arr {
                Some(arr) => Value::Array(arr),
                None => Value::Null(()),
            })
        }
        '_' => Ok(Value::Null(parse_null(stream)?)),
        '#' => Ok(Value::Bool(parse_bool(stream)?)),
        ',' => Ok(Value::Double(parse_double(stream)?)),
        '(' => Ok(Value::BigNumber(parse_big_number(stream)?)),
        '!' => {
            let bstring = parse_bulk_string(stream)?;
            Ok(match bstring {
                Some(bstring) => Value::BulkError(bstring),
                None => Value::Null(()),
            })
        }
        '=' => Ok(Value::VerbatimString(parse_verbatim_string(stream)?)),
        '%' => Ok(Value::Map(parse_map(stream)?)),
        '~' => Ok(Value::Set(parse_set(stream)?)),
        '>' => {
            let arr = parse_array(stream)?;
            Ok(match arr {
                Some(arr) => Value::Push(arr),
                None => Value::Null(()),
            })
        }
        datatype => Err(ParseError::UnknownDataType(format!(
            "ERROR: unknown datatype {:?}",
            datatype
        ))),
    }
}

pub fn parse_simple_string(stream: &mut dyn Read) -> Result<String, ParseError> {
    let mut ret = Vec::new();
    let mut buf = [1];
    while let Ok(_) = stream.read(&mut buf) {
        if buf[0] as char == '\r' {
            let _ = stream.read(&mut buf);
            break;
        }
        ret.push(buf[0]);
    }
    String::from_utf8(ret).map_err(|err| ParseError::SimpleStringParseError(err.to_string()))
}

pub fn parse_integer(stream: &mut dyn Read) -> Result<i64, ParseError> {
    let int_as_str = parse_simple_string(stream)?;
    int_as_str
        .parse()
        .map_err(|err: std::num::ParseIntError| ParseError::IntegerParseError(err.to_string()))
}

pub fn parse_bulk_string(stream: &mut dyn Read) -> Result<Option<String>, ParseError> {
    let string_len = parse_integer(stream)?;
    if string_len == -1 {
        return Ok(None);
    }
    let string = parse_simple_string(stream)?;
    Ok(Some(string))
}

pub fn parse_array(stream: &mut dyn Read) -> Result<Option<Vec<Value>>, ParseError> {
    let n = parse_integer(stream)?;
    if n == -1 {
        return Ok(None);
    }
    let mut ret = Vec::new();
    for _ in 0..n {
        ret.push(parse(stream)?);
    }
    Ok(Some(ret))
}

pub fn parse_null(stream: &mut dyn Read) -> Result<(), ParseError> {
    let mut buf = [1];
    let _ = stream.read(&mut buf);
    let _ = stream.read(&mut buf);
    Ok(())
}

pub fn parse_bool(stream: &mut dyn Read) -> Result<bool, ParseError> {
    let mut buf = [1];
    let b = stream.read(&mut buf);
    if b.is_ok() {
        let mut temp_buf = [1];
        let _ = stream.read(&mut temp_buf);
        let _ = stream.read(&mut temp_buf);
        return if buf[0] == b't' { Ok(true) } else { Ok(false) };
    } else {
        b.map_err(|err| ParseError::BoolParseError(err.to_string()))
            .map(|_| false)
    }
}

pub fn parse_double(stream: &mut dyn Read) -> Result<MyFloat, ParseError> {
    let f_as_str = parse_simple_string(stream)?;
    f_as_str
        .parse::<f64>()
        .map_err(|err: std::num::ParseFloatError| ParseError::DoubleParseError(err.to_string()))
        .map(|f| MyFloat::Real(f))
}

//TODO: create a custom struct to represent a big number
pub fn parse_big_number(stream: &mut dyn Read) -> Result<String, ParseError> {
    parse_simple_string(stream)
}

pub fn parse_verbatim_string(stream: &mut dyn Read) -> Result<VerbatimString, ParseError> {
    let binding = parse_simple_string(stream)?;
    let mut raw_str = binding.split(':');
    let enc = raw_str
        .next()
        .ok_or(ParseError::VerbatimStringParseError(
            "Encoding Error, no ':' delimiter between encoding type and data".to_string(),
        ))?
        .to_string();
    let data = raw_str
        .next()
        .ok_or(ParseError::VerbatimStringParseError(
            "Encoding Error, no ':' delimiter between encoding type and data".to_string(),
        ))?
        .to_string();
    Ok(VerbatimString { enc, data })
}

pub fn parse_map(stream: &mut dyn Read) -> Result<BTreeMap<Value, Value>, ParseError> {
    let n = parse_integer(stream)?;
    let mut ret = BTreeMap::new();
    for _ in 0..n {
        let key = parse(stream)?;
        let value = parse(stream)?;
        ret.insert(key, value);
    }
    Ok(ret)
}

pub fn parse_set(stream: &mut dyn Read) -> Result<BTreeSet<Value>, ParseError> {
    let n = parse_integer(stream)?;
    let mut ret = BTreeSet::new();
    for _ in 0..n {
        let value = parse(stream)?;
        ret.insert(value);
    }
    Ok(ret)
}
