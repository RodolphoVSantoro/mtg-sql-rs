#![allow(clippy::needless_return, clippy::manual_let_else)]

use sqlx::{Pool, Sqlite};

pub mod db;
pub mod responses;
pub mod query_builder;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}
