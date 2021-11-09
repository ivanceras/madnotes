use crate::server::page;
use crate::server::ServiceError;
use client::Settings;
use sauron::prelude::*;

#[derive(Debug)]
pub struct RawResponse<'a> {
    pub content: Vec<u8>,
    pub headers: Vec<(&'a str, String)>,
}

impl<'a> RawResponse<'a> {
    fn new(content: Vec<u8>, headers: Vec<(&'a str, String)>) -> Self {
        Self { content, headers }
    }
}

pub(crate) fn client_bg_wasm_content() -> Vec<u8> {
    include_bytes!("../client/pkg/client_bg.wasm").to_vec()
}

pub(crate) fn favicon_ico_content() -> Vec<u8> {
    include_bytes!("../favicon.ico").to_vec()
}
pub(crate) fn client_js_content() -> &'static str {
    include_str!("../client/pkg/client.js")
}

pub(crate) fn font_content() -> Vec<u8> {
    include_bytes!("../assets/JuliaMono-Light.woff2").to_vec()
}

pub fn raw_serve<'a>(
    settings: &Settings,
    path_and_query: &str,
) -> Result<RawResponse<'a>, ServiceError> {
    match &*path_and_query {
        "/" => {
            let index_page = page::index(settings).render_to_string();
            Ok(RawResponse::new(index_page.into(), vec![]))
        }
        "/favicon.ico" => Ok(RawResponse::new(
            favicon_ico_content().into(),
            vec![("Content-Type", "image/x-icon".to_string())],
        )),
        "/pkg/client.js" => Ok(RawResponse::new(
            client_js_content().into(),
            vec![("Content-Type", "text/javascript; charset=UTF-8".to_string())],
        )),
        "/pkg/client_bg.wasm" => Ok(RawResponse::new(
            client_bg_wasm_content().into(),
            vec![("Content-Type", "application/wasm".to_string())],
        )),
        "/assets/JuliaMono-Light.woff2" => Ok(RawResponse::new(
            font_content().into(),
            vec![("Content-Type", "font/woff2".to_string())],
        )),
        _ => Err(ServiceError::NotFound),
    }
}
