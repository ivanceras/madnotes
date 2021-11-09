use crate::serve_files;
use client::Settings;
use hyper::header::HeaderName;
use hyper::header::HeaderValue;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use percent_encoding::percent_decode_str;
use serde::Serialize;
use std::borrow::Cow;
use std::net::SocketAddr;
use std::str::FromStr;
use thiserror::Error;
use tokio::sync::oneshot::{Receiver, Sender};

const DEFAULT_IP: [u8; 4] = [127, 0, 0, 1];
const DEFAULT_PORT: u16 = 0; //the server will choose an unused port

pub(crate) mod page;

#[derive(Error, Debug, Serialize)]
pub enum ServiceError {
    #[error("NotFound")]
    NotFound,
}

impl ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

pub(crate) fn extract_path_and_query<T>(request: &Request<T>) -> Cow<'_, str> {
    let pnq = request
        .uri()
        .path_and_query()
        .map(|pnq| pnq.as_str())
        .unwrap_or("/");
    percent_decode_str(pnq).decode_utf8_lossy()
}

pub fn create_response(
    status_code: StatusCode,
    body: Body,
    headers: Vec<(&str, String)>,
) -> Response<Body> {
    let mut resp = Response::builder()
        .status(status_code)
        .header("Accept", "*/*")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Expose-Headers", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Credentials", "true")
        .body(body)
        .expect("must build");
    let headers_mut = resp.headers_mut();
    for (header, value) in headers.iter() {
        headers_mut.append(
            HeaderName::from_str(header).expect("valud header"),
            HeaderValue::from_str(value).expect("valid value"),
        );
    }
    resp
}

pub fn error_response(e: ServiceError) -> Response<Body> {
    let status_code = e.status_code();
    let error_json = serde_json::to_string_pretty(&e).expect("must serialize");
    create_response(status_code, error_json.into(), vec![])
}

/// Let the server decide the port
/// This is used by the main server
#[allow(unused)]
pub(crate) async fn serve_ephemeral(
    settings: Settings,
    socket_tx: Sender<SocketAddr>,
    shutdown_rx: Receiver<()>,
) {
    serve(settings, None, None, socket_tx, shutdown_rx).await;
}

fn serve_request(settings: &Settings, request: &Request<Body>) -> Response<Body> {
    let path_and_query = extract_path_and_query(request);
    match serve_files::raw_serve(settings, &*path_and_query) {
        Ok(raw_response) => create_response(
            StatusCode::OK,
            raw_response.content.into(),
            raw_response.headers,
        ),
        Err(e) => error_response(e),
    }
}

async fn serve_request_wrap(
    settings: Settings,
    request: Request<Body>,
) -> Result<Response<Body>, ServiceError> {
    Ok(serve_request(&settings, &request))
}

pub(crate) async fn serve(
    settings: Settings,
    ip: Option<[u8; 4]>,
    port: Option<u16>,
    socket_tx: Sender<SocketAddr>,
    shutdown_rx: Receiver<()>,
) {
    let port = port.unwrap_or(DEFAULT_PORT);
    let ip = ip.unwrap_or(DEFAULT_IP);
    let socket: SocketAddr = (ip, port).into();

    let server = Server::bind(&socket).serve(make_service_fn(move |_| {
        let settings = settings.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let settings = settings.clone();
                serve_request_wrap(settings, req)
            }))
        }
    }));
    let local_socket = server.local_addr();
    socket_tx
        .send(local_socket)
        .expect("must be able to send the socket address");

    println!("http://{}", local_socket);

    let graceful = server.with_graceful_shutdown(async {
        shutdown_rx.await.ok();
    });

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
