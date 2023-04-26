#![allow(clippy::invalid_regex, dead_code)]

use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

use super::{http_header::HttpHeader, http_method::HttpMethod, request_param::RequestParam};

#[derive(Debug)]
pub enum RequestParsingError {
    NonHttpRequest,
    InvalidHttpMethod,
}

#[derive(Debug)]
pub struct Request {
    pub line: String,
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String,
    pub query_params: Vec<RequestParam>,
    pub route_params: Vec<RequestParam>,
    pub headers: Vec<HttpHeader>,
    pub body: Option<String>,
}

impl Request {
    ///
    /// Get a query paramater by its name and type
    ///
    /// It returns None if no param exist with the same name
    /// or if the param exist but can not be parsed to the specified type T
    ///
    pub fn get_query_param<T: FromStr>(&self, name: &str) -> Option<T> {
        match self.query_params.iter().find(|param| param.key == name) {
            Some(param) => match param.parse::<T>() {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn parse(stream: &TcpStream) -> Result<Self, RequestParsingError> {
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
        let full_path = line_parts.next().unwrap().to_owned();
        let version = line_parts.next().unwrap().to_owned();

        let base_path = Self::parse_base_path(full_path.as_str());
        let query_params = Self::parse_query_params(full_path.as_str());

        Ok(Self {
            line: request_line,
            method,
            path: base_path,
            http_version: version,
            query_params,
            route_params: vec![],
            headers: vec![],
            body: None,
        })
    }

    fn parse_query_params(path: &str) -> Vec<RequestParam> {
        let mut query_params = vec![];
        if let Some((_, query)) = path.split_once('?') {
            query_params = query
                .split('&')
                .filter_map(|param| param.split_once('='))
                .map(|(k, v)| RequestParam {
                    key: k.to_owned(),
                    value: v.to_owned(),
                })
                .collect();
        }

        query_params
    }

    fn parse_base_path(path: &str) -> String {
        match path.split_once('?') {
            Some((new_path, _)) => new_path.to_owned(),
            None => path.to_owned(),
        }
    }
}
