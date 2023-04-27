#![allow(clippy::invalid_regex, dead_code)]

use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

use crate::route::Route;

use super::{http_header::HttpHeader, http_method::HttpMethod, request_param::RequestParam};

#[derive(Debug, Clone, Copy)]
pub enum RequestParsingError {
    NonHttpRequest,
    InvalidHttpMethod,
}

#[derive(Debug)]
pub struct Request {
    /// The request line (ex; GET /home HTTP/1.1) is the first line in HTTP request
    pub line: String,
    pub method: HttpMethod,
    pub http_version: String,

    /// The request base path, without query parametres
    pub base_path: String,

    /// The request full path, without query parametres
    pub full_path: String,

    /// list of request query parametres
    pub query_params: Vec<RequestParam>,

    /// list of request route parametres
    pub route_params: Vec<RequestParam>,

    /// list of request headers as (key, value) paires
    pub headers: Vec<HttpHeader>,

    /// The request body
    /// it's of type Option becuase some request does not have a body like GET, DELETE
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

    ///
    /// Get a route paramater by its name and type
    ///
    /// It returns None if no param exist with the same name
    /// or if the param exist but can not be parsed to the specified type T
    ///
    pub fn get_route_param<T: FromStr>(&self, name: &str) -> Option<T> {
        match self.route_params.iter().find(|param| param.key == name) {
            Some(param) => match param.parse::<T>() {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            None => None,
        }
    }

    ///
    /// Parse the request basic information like method, version, base_path..
    ///
    /// This method will not parse request params, headers, or body these information
    /// will be parsed after finding a matching route using `complete_parsing` method.
    ///
    pub fn initial_parse(stream: &TcpStream) -> Result<Self, RequestParsingError> {
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

        // This may return InvalidHttpMethod error
        let method = HttpMethod::try_from(line_parts.next().unwrap())?;

        let full_path = line_parts.next().unwrap().to_owned();
        let base_path = Self::parse_base_path(full_path.as_str());
        let version = line_parts.next().unwrap().to_owned();

        Ok(Self {
            line: request_line,
            method,
            base_path,
            full_path,
            http_version: version,
            query_params: vec![],
            route_params: vec![],
            headers: vec![],
            body: None,
        })
    }

    ///
    /// This method will parse request params, headers, and body
    /// then append theme to the current request object `self`
    ///
    pub fn complete_parsing(
        &mut self,
        _stream: &TcpStream,
        matched_route: &Route,
    ) -> Result<(), RequestParsingError> {
        self.query_params = self.parse_query_params();
        self.route_params = self.parse_route_params(matched_route);

        Ok(())
    }

    fn parse_route_params(&self, matched_route: &Route) -> Vec<RequestParam> {
        let params = matched_route.get_params();
        let param_indexs: Vec<_> = params.iter().map(|param| param.0).collect();

        self.base_path
            .split('/')
            .enumerate()
            .filter(|(i, _)| param_indexs.contains(i))
            .enumerate()
            .map(|(i, (_, val))| RequestParam {
                key: params[i].1.to_owned(),
                value: val.to_owned(),
            })
            .collect()
    }

    fn parse_query_params(&self) -> Vec<RequestParam> {
        let mut query_params = vec![];
        if let Some((_, query)) = self.full_path.split_once('?') {
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
