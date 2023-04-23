#![allow(clippy::invalid_regex)]

use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use lazy_static::lazy_static;
use regex::Regex;

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl TryFrom<&str> for HttpMethod {
    type Error = RequestParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "PATCH" => Ok(HttpMethod::Patch),
            "DELETE" => Ok(HttpMethod::Delete),
            _ => Err(self::RequestParsingError::InvalidHttpMethod),
        }
    }
}

pub struct Request {
    pub line: String,
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String,
}

#[derive(Debug)]
pub enum RequestParsingError {
    NonHttpRequest,
    InvalidHttpMethod,
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

        let mut line_parts = request_line.split_whitespace();
        let method = HttpMethod::try_from(line_parts.next().unwrap())?;
        let path = line_parts.next().unwrap().to_owned();
        let version = line_parts.next().unwrap().to_owned();

        Ok(Self {
            line: request_line,
            method,
            path,
            http_version: version,
        })
    }
}
