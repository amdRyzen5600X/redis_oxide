use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

pub mod parse;

#[derive(Debug, Clone)]
pub enum ParseError {
    SimpleStringParseError(String),
    IntegerParseError(String),
    BoolParseError(String),
    DoubleParseError(String),
    VerbatimStringParseError(String),
    UnknownDataType(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerbatimString {
    pub enc: String,
    pub data: String,
}

impl ToString for VerbatimString {
    fn to_string(&self) -> String {
        format!("{}:{}", self.enc, self.data)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    String(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<Value>),
    Null(()),
    Bool(bool),
    Double(MyFloat),
    BigNumber(String),
    BulkError(String),
    VerbatimString(VerbatimString),
    Map(BTreeMap<Value, Value>),
    Set(BTreeSet<Value>),
    Push(Vec<Value>),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.to_string(),
            Value::Error(s) => s.to_string(),
            Value::Integer(i) => i.to_string(),
            Value::BulkString(s) => s.to_string(),
            Value::Null(()) => "None".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Double(d) => d.to_string(),
            Value::BigNumber(s) => s.to_string(),
            Value::BulkError(s) => s.to_string(),
            Value::VerbatimString(s) => s.to_string(),
            _ => "ERROR".to_string(),
        }
    }
}

impl Value {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        match self {
            Value::String(s) => {
                ret.push('+' as u8);
                ret.extend_from_slice(s.as_bytes());
                ret.extend_from_slice(b"\r\n");
            }
            Value::Error(s) => {
                ret.push('-' as u8);
                ret.extend_from_slice(s.as_bytes());
                ret.extend_from_slice(b"\r\n");
            }
            Value::Integer(i) => {
                ret.push(':' as u8);
                ret.extend_from_slice(i.to_string().as_bytes());
                ret.extend_from_slice(b"\r\n");
            }
            Value::BulkString(s) => {
                ret.push('$' as u8);
                ret.extend_from_slice(s.len().to_string().as_bytes());
                ret.extend_from_slice(b"\r\n");
                ret.extend_from_slice(s.as_bytes());
                ret.extend_from_slice(b"\r\n");
            }
            Value::Array(a) => {
                ret.push('*' as u8);
                ret.extend_from_slice(a.len().to_string().as_bytes());
                ret.extend_from_slice(b"\r\n");
                for v in a {
                    ret.extend_from_slice(&v.to_bytes().as_slice());
                }
            }
            //TODO: null BulkString and null Array
            Value::Null(_) => {
                ret.extend_from_slice(b"_\r\n");
            }
            Value::Bool(b) => {
                ret.push('#' as u8);
                if *b {
                    ret.push('t' as u8)
                } else {
                    ret.push('f' as u8)
                }
                ret.extend_from_slice(b"\r\n");
            }
            Value::Double(d) => {
                ret.push(',' as u8);
                ret.extend_from_slice(d.to_string().as_bytes());
                ret.extend_from_slice(b"\r\n");
            }
            Value::BigNumber(b) => {
                ret.push('(' as u8);
                ret.extend_from_slice(b.as_bytes());
                ret.extend_from_slice(b"\r\n");
            }
            Value::BulkError(e) => {
                ret.push('!' as u8);
                ret.extend_from_slice(e.len().to_string().as_bytes());
                ret.extend_from_slice(b"\r\n");
                ret.extend_from_slice(e.as_bytes());
                ret.extend_from_slice(b"\r\n");
            }
            Value::VerbatimString(s) => {
                ret.push('=' as u8);
                let data = s.to_string();
                ret.extend_from_slice(data.len().to_string().as_bytes());
                ret.extend_from_slice(b"\r\n");
                ret.extend_from_slice(data.as_bytes());
                ret.extend_from_slice(b"\r\n");
            }
            Value::Map(m) => {
                ret.push('%' as u8);
                ret.extend_from_slice(m.len().to_string().as_bytes());
                ret.extend_from_slice(b"\r\n");
                for (k, v) in m.iter() {
                    ret.extend_from_slice(&k.to_bytes());
                    ret.extend_from_slice(&v.to_bytes());
                }
            }
            Value::Set(s) => {
                ret.push('~' as u8);
                ret.extend_from_slice(s.len().to_string().as_bytes());
                ret.extend_from_slice(b"\r\n");
                for v in s.iter() {
                    ret.extend_from_slice(&v.to_bytes());
                }
            }
            Value::Push(p) => {
                ret.push('>' as u8);
                ret.extend_from_slice(p.len().to_string().as_bytes());
                ret.extend_from_slice(b"\r\n");
                for v in p {
                    ret.extend_from_slice(&v.to_bytes().as_slice());
                }
            }
        }
        ret
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MyFloat {
    Real(f64),
    NaN,
}

impl ToString for MyFloat {
    fn to_string(&self) -> String {
        match self {
            Self::Real(f) => f.to_string(),
            Self::NaN => "0.0".to_string(),
        }
    }
}

impl Eq for MyFloat {}

impl Ord for MyFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self > other {
            return std::cmp::Ordering::Greater;
        } else if self < other {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Equal;
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        return if self > other { self } else { other };
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        return if self < other { self } else { other };
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min < max);
        if self > max {
            return max;
        } else if self < min {
            return min;
        }
        return self;
    }
}
