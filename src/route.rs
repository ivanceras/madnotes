use crate::server;
use hyper::{Body, Method, Request};
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Route {
    pub method: String,
    pub body: Option<String>,
    pub url: String,
    pub callback_id: usize,
    pub headers: Option<HashMap<String, String>>,
}

impl Route {
    pub fn to_request(&self) -> Request<Body> {
        let body = if let Some(body) = &self.body {
            Body::from(body.to_string())
        } else {
            Body::from(String::new())
        };

        let method = Method::from_str(&self.method).expect("method name must be correct");

        Request::builder()
            .uri(&self.url)
            .method(method)
            .body(body)
            .expect("must be converted to request")
    }

    pub fn path_and_query(&self) -> String {
        let request = self.to_request();
        server::extract_path_and_query(&request).to_string()
    }
}
