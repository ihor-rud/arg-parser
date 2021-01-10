use std::fmt::Debug;
use std::iter::Peekable;
use std::str::FromStr;
use std::{collections::HashMap, convert::TryFrom};

#[derive(PartialEq, Eq, Hash, Debug)]
enum ArgName {
    Long(String),
    Short(String),
}

enum Choice {
    Long,
    Short,
    None,
}

impl ArgName {
    fn is_name(val: &str) -> Choice {
        if val.starts_with("--") && val.chars().count() > 2 {
            Choice::Long
        } else if val.starts_with("-")
            && val.chars().count() == 2
            && val
                .chars()
                .skip(1)
                .all(|c| c.is_alphabetic())
        {
            Choice::Short
        } else {
            Choice::None
        }
    }
}

impl TryFrom<String> for ArgName {
    type Error = &'static str;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        match ArgName::is_name(&val) {
            Choice::Long => Ok(ArgName::Long(val)),
            Choice::Short => Ok(ArgName::Short(val)),
            Choice::None => Err("Cannot parse value into ArgName"),
        }
    }
}

#[derive(Debug)]
pub enum ArgValue {
    None,
    One(String),
    Many(Vec<String>),
}

impl ArgValue {
    fn append(self, val: String) -> Self {
        match self {
            ArgValue::None => ArgValue::One(val),
            ArgValue::One(prev_arg) => ArgValue::Many(vec![prev_arg, val]),
            ArgValue::Many(mut arg_values) => {
                arg_values.push(val);
                ArgValue::Many(arg_values)
            }
        }
    }
}

struct ArgIter<T>
where
    T: Iterator<Item = String>,
{
    iter: Peekable<T>,
}

impl<T> ArgIter<T>
where
    T: Iterator<Item = String>,
{
    fn new(iter: T) -> Self {
        ArgIter { iter: iter.peekable() }
    }
}

impl<T> Iterator for ArgIter<T>
where
    T: Iterator<Item = String>,
{
    type Item = (ArgName, ArgValue);

    fn next(&mut self) -> Option<Self::Item> {
        let arg_name = match self.iter.next() {
            Some(val) => ArgName::try_from(val).unwrap(),
            None => return None,
        };

        let mut arg_value = ArgValue::None;
        loop {
            match self.iter.peek() {
                Some(arg) => match ArgName::is_name(arg) {
                    Choice::None => {
                        arg_value = arg_value.append(self.iter.next().unwrap());
                    }
                    Choice::Long | Choice::Short => break,
                },
                None => break,
            }
        }
        Some((arg_name, arg_value))
    }
}

pub fn collect_args<T>(env_args: T) -> HashMap<String, ArgValue>
where
    T: Iterator<Item = String>,
{
    ArgIter::new(env_args.skip(1))
        .map(|(k, v)| {
            (
                match k {
                    ArgName::Short(k) => k,
                    ArgName::Long(k) => k,
                },
                v,
            )
        })
        .collect()
}

pub trait ToType<T> {
    fn parse(&self) -> T;

    fn parse_option(val: Option<&Self>) -> Option<T> {
        Some(val?.parse())
    }

    fn reduce(left: Option<T>, right: Option<T>) -> T {
        match (left, right) {
            (None, None) => panic!(),
            (None, Some(val)) => val,
            (Some(val), None) => val,
            (Some(_), Some(_)) => panic!(),
        }
    }
}

impl ToType<String> for ArgValue {
    fn parse(&self) -> String {
        match self {
            ArgValue::None => panic!(),
            ArgValue::One(val) => String::from_str(val).unwrap(),
            ArgValue::Many(val) => {
                dbg!(val);
                panic!()
            }
        }
    }
}

impl ToType<i32> for ArgValue {
    fn parse(&self) -> i32 {
        match self {
            ArgValue::None => panic!(),
            ArgValue::One(val) => i32::from_str(val).unwrap(),
            ArgValue::Many(_) => panic!(),
        }
    }
}

impl ToType<bool> for ArgValue {
    fn parse(&self) -> bool {
        match self {
            ArgValue::None => true,
            ArgValue::One(val) => bool::from_str(val).unwrap(),
            ArgValue::Many(_) => panic!(),
        }
    }

    fn parse_option(val: Option<&Self>) -> Option<bool> {
        match val {
            Some(val) => Some(val.parse()),
            None => Some(false),
        }
    }

    fn reduce(left: Option<bool>, right: Option<bool>) -> bool {
        match (left, right) {
            (None, None) => false,
            (None, Some(val)) => val,
            (Some(val), None) => val,
            (Some(left), Some(right)) => left || right,
        }
    }
}

impl<T> ToType<Vec<T>> for ArgValue
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    fn parse(&self) -> Vec<T> {
        match self {
            ArgValue::None => Vec::default(),
            ArgValue::One(val) => vec![T::from_str(val).unwrap()],
            ArgValue::Many(values) => values
                .iter()
                .map(|val| T::from_str(val).unwrap())
                .collect(),
        }
    }

    fn reduce(left: Option<Vec<T>>, right: Option<Vec<T>>) -> Vec<T> {
        match (left, right) {
            (None, None) => panic!(),
            (None, Some(val)) => val,
            (Some(val), None) => val,
            (Some(mut left), Some(right)) => {
                left.extend(right);
                left
            }
        }
    }
}

impl<T> ToType<Option<T>> for ArgValue
where
    Self: ToType<T>,
{
    fn parse(&self) -> Option<T> {
        match ArgValue::parse_option(Some(self)) {
            Some(val) => Some(val),
            None => panic!(),
        }
    }

    fn reduce(left: Option<Option<T>>, right: Option<Option<T>>) -> Option<T> {
        match (left, right) {
            (None, None) => None,
            (None, Some(val)) => val,
            (Some(val), None) => val,
            (Some(left), Some(right)) => Some(ArgValue::reduce(left, right)),
        }
    }
}

#[macro_export]
macro_rules! command_args_parser {
    (@private $($var:ident as $type:ty where {short: $short:literal, long: $long:literal });+ $args:expr) => {
        {
            #[derive(Debug)]
            struct Args {
                $($var: $type),+,
            }
            let args = $crate::collect_args($args);
            Args {
                $($var: $crate::ArgValue::reduce(
                    $crate::ArgValue::parse_option(args.get(concat!("-", $short))),
                    $crate::ArgValue::parse_option(args.get(concat!("--", $long))),
                )),+,
            }
        }
    };
    ($($var:ident as $type:ty where {short: $short:literal, long: $long:literal });+;) => {
        command_args_parser!(@private $($var as $type where {short: $short, long: $long });+ ::std::env::args())
    };
}
