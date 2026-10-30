#![allow(clippy::needless_return, clippy::manual_let_else)]

use sqlx::{Pool, Sqlite};

pub mod query_builder;
pub mod responses;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}
