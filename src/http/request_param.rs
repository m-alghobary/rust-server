use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub struct RequestParam {
    pub key: String,
    pub value: String,
}

impl RequestParam {
    pub fn parse<T: FromStr>(&self) -> Result<T, ParamParsingError> {
        match self.value.parse::<T>() {
            Ok(val) => Ok(val),
            Err(_) => Err(ParamParsingError),
        }
    }
}

pub struct ParamParsingError;

impl Display for ParamParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Parameter parsing failed!"))
    }
}
