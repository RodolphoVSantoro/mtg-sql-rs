#![allow(clippy::needless_return)]

use axum::{
    Json,
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum HttpError {
    InternalError(String),
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct MessageResponse {
    pub message: String,
}

pub fn internal_server_response() -> Response<Body> {
    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(MessageResponse {
            message: "internal server error".to_string(),
        }),
    )
        .into_response();
}

pub fn ok_response<T: Serialize>(data: T) -> Response<Body> {
    return (StatusCode::OK, Json(data)).into_response();
}
