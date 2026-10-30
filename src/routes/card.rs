#![allow(clippy::needless_return, clippy::manual_let_else)]

use crate::{
    helpers::{
        AppState,
        responses::{internal_server_response, ok_response},
    },
    models::card,
    repositories::card::{CardFilter, CardHttpQuery},
};

use axum::{
    Json, Router,
    body::Body,
    extract::{Query, State},
    response::Response,
};
use serde::{Deserialize, Serialize};

pub async fn list(
    State(app_state): State<AppState>,
    query: Option<Query<CardHttpQuery>>,
    body: Option<Json<CardFilter>>,
) -> Response<Body> {
    let conn = &mut app_state.pool.acquire().await;

    let conn = match conn {
        Ok(conn) => conn,
        Err(e) => {
            println!("error getting connection: {e:?}");
            return internal_server_response();
        }
    };

    let cards_result = card::CardSimple::list(
        conn,
        body.map(|Json(filter)| filter),
        query.map(|Query(card_query)| card_query),
    )
    .await;
    match cards_result {
        Ok(cards_result) => {
            return ok_response(CardResult {
                count: cards_result.cards.iter().count(),
                total: cards_result.total,
                data: cards_result
                    .cards
                    .into_iter()
                    .map(|card| Card::Simple(card))
                    .collect(),
            });
        }
        Err(e) => {
            println!("error listing cards: {e:?}");
            return internal_server_response();
        }
    }
}

pub async fn list_complete(
    State(app_state): State<AppState>,
    query: Option<Query<CardHttpQuery>>,
    body: Option<Json<CardFilter>>,
) -> Response<Body> {
    let conn = &mut app_state.pool.acquire().await;

    let conn = match conn {
        Ok(conn) => conn,
        Err(e) => {
            println!("error getting connection: {e:?}");
            return internal_server_response();
        }
    };

    let cards_result = card::CardComplete::list(conn, body.map(|Json(filter)| filter),
        query.map(|Query(card_query)| card_query),).await;

    match cards_result {
        Ok(cards_result) => {
            return ok_response(CardResult {
                count: cards_result.cards.iter().count(),
                total: cards_result.total,
                data: cards_result
                    .cards
                    .into_iter()
                    .map(|card| Card::Complete(card))
                    .collect(),
            });
        }
        Err(e) => {
            println!("error listing cards: {e:?}");
            return internal_server_response();
        }
    }
}

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", axum::routing::post(list))
        .route("/complete", axum::routing::post(list_complete));
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
enum Card {
    Simple(card::CardSimple),
    Complete(card::CardComplete),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardResult {
    count: usize,
    total: i32,
    data: Vec<Card>,
}
