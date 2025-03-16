use std::collections::{BTreeMap, BTreeSet};

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


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    String(String),
    Error(String),
    Integer(i64),
    Array(Vec<Value>),
    Null(()),
    Bool(bool),
    Double(MyFloat),
    BigNumber(String),
    VerbatimString(VerbatimString),
    Map(BTreeMap<Value, Value>),
    Set(BTreeSet<Value>),
    Push(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MyFloat {
    Real(f64),
    NaN,
}

impl Eq for MyFloat {
}

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
            Self: Sized, {
        return if self > other { self } else { other };
    }

    fn min(self, other: Self) -> Self
        where
            Self: Sized, {
        return if self < other { self } else { other };
    }

    fn clamp(self, min: Self, max: Self) -> Self
        where
            Self: Sized, {
        assert!(min < max);
        if self > max {
            return max;
        } else if self < min {
            return min;
        }
        return self;
    }
}
