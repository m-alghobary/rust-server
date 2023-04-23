#![allow(clippy::invalid_regex)]

use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use lazy_static::lazy_static;
use regex::Regex;

pub struct Request {
    pub line: String,
}

#[derive(Debug)]
pub enum RequestParsingError {
    NonHttpRequest,
}

impl TryFrom<&TcpStream> for Request {
    type Error = RequestParsingError;

    fn try_from(stream: &TcpStream) -> Result<Self, Self::Error> {
        let request_line = BufReader::new(stream).lines().next().unwrap().unwrap();

        // this is used to ensure that regular expression is compiled exactly once
        lazy_static! {
            static ref HTTP_REGEX: Regex =
                Regex::new(r"^(GET|POST|PUT|DELETE|PATCH)\s\/.*\sHTTP\/").unwrap();
        }

        if !HTTP_REGEX.is_match(&request_line) {
            return Err(self::RequestParsingError::NonHttpRequest);
        }

        Ok(Self { line: request_line })
    }
}
