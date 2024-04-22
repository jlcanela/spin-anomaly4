use spin_sdk::http;
use spin_sdk::http::Response;
use anyhow::Result;
use super::*;
//# utils.rs
// use super::*;

    
struct StatusCode {
}

impl StatusCode {
    pub const OK: u16 = 200;
    pub const METHOD_NOT_ALLOWED: u16 = 405; 
    pub const BAD_REQUEST: u16 = 400;
    pub const NOT_FOUND: u16 = 404;
    pub const NO_CONTENT: u16 = 204;
    pub const INTERNAL_SERVER_ERROR: u16 = 500;
}

struct Header {
}

impl Header {
    pub const CONTENT_TYPE: &'static str = "Content-Type";
}

pub(crate) fn internal_server_error(err: String) -> Result<Response> {
  Ok(http::Response::builder()
    .status(StatusCode::INTERNAL_SERVER_ERROR)
    .header(Header::CONTENT_TYPE, "text/plain")
    .body(err).build())
}

pub(crate) fn json<T>(payload: &T) -> Result<Response>
    where
    T: ?Sized + Serialize
     {
    let json = serde_json::to_string(payload)?;

    Ok(http::Response::builder()
      .status(StatusCode::OK)
      .header(Header::CONTENT_TYPE, "application/json")
      .body(json).build())
}

pub(crate) fn ok(payload: String) -> Result<Response> {
  Ok(http::Response::builder()
    .status(StatusCode::OK)
    .header(Header::CONTENT_TYPE, "application/json")
    .body(payload).build())
}

pub(crate) fn method_not_allowed() -> Result<Response> {
  quick_response(StatusCode::METHOD_NOT_ALLOWED)
}

pub(crate) fn bad_request() -> Result<Response> {
  quick_response(StatusCode::BAD_REQUEST)
}

pub(crate) fn not_found() -> Result<Response> {
  quick_response(StatusCode::NOT_FOUND)
}

pub(crate) fn no_content() -> Result<Response> {
  quick_response(StatusCode::NO_CONTENT)
}

fn quick_response(s: http::StatusCode) -> Result<Response> {
  Ok(http::Response::builder().status(s).body(()).build())
}
