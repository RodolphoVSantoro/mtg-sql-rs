use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Sqlite, pool::PoolConnection};

use crate::helpers::query_builder::SqlQueryBuilder;
use crate::helpers::responses::HttpError;

use std::collections::HashMap;

pub type CardFilter = HashMap<String, Value>;

const STR_LIKE_FILTERS: [&str; 4] = ["name", "text", "flavorText", "manaCost"];
const STR_NOT_LIKE_FILTERS: [&str; 4] = ["nameNot", "textNot", "flavorTextNot", "manaCostNot"];

const STR_EQUAL_FILTERS: [&str; 2] = ["artist", "borderColor"];

const NUM_EQUAL_FILTERS: [&str; 4] = ["manaValue", "power", "toughness", "loyalty"];

fn add_str_like_filter<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    like_filter: &str,
    like_filter_value: &str,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    builder
        .builder
        .push(format!("lower({}) like ", like_filter))
        .push_bind(format!("%{}%", like_filter_value.to_lowercase()));

    count_builder
        .builder
        .push(format!("lower({}) like ", like_filter))
        .push_bind(format!("%{}%", like_filter_value.to_lowercase()));
    return (builder, count_builder);
}

fn add_str_like_filters<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: &CardFilter,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    for like_filter in STR_LIKE_FILTERS {
        if let Some(like_filter_value) = filter
            .get(like_filter)
            .and_then(|v| v.as_str())
            .map(|s| s as &str)
        {
            builder.add_condition();
            count_builder.add_condition();
            (builder, count_builder) =
                add_str_like_filter(builder, count_builder, like_filter, like_filter_value);
        };
        if let Some(like_filter_values) =
            filter.get(like_filter).and_then(|v| v.as_array()).map(|s| {
                s.into_iter()
                    .map(|v| v.as_str())
                    .filter(|v| v.is_some())
                    .map(|v| v.unwrap())
                    .collect::<Vec<&str>>()
            })
        {
            builder.add_condition();
            count_builder.add_condition();
            builder.begin_multi_filter();
            count_builder.begin_multi_filter();
            for like_filter_value in like_filter_values {
                builder.add_multi_filter();
                count_builder.add_multi_filter();
                (builder, count_builder) =
                    add_str_like_filter(builder, count_builder, like_filter, like_filter_value);
            }
            builder.end_multi_filter();
            count_builder.end_multi_filter();
        };
    }

    return (builder, count_builder);
}

fn add_str_not_like_filter<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    not_like_filter: &str,
    not_like_filter_value: &str,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    builder
        .builder
        .push(format!(
            "lower({}) not like ",
            &not_like_filter[..not_like_filter.len() - 3]
        ))
        .push_bind(format!("%{}%", not_like_filter_value.to_lowercase()));

    count_builder
        .builder
        .push(format!(
            "lower({}) not like ",
            &not_like_filter[..not_like_filter.len() - 3]
        ))
        .push_bind(format!("%{}%", not_like_filter_value.to_lowercase()));
    return (builder, count_builder);
}

fn add_str_not_like_filters<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: &CardFilter,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    for not_like_filter in STR_NOT_LIKE_FILTERS {
        if let Some(not_like_filter_value) = filter
            .get(not_like_filter)
            .and_then(|v| v.as_str())
            .map(|s| s as &str)
        {
            builder.add_condition();
            count_builder.add_condition();
            (builder, count_builder) = add_str_not_like_filter(
                builder,
                count_builder,
                not_like_filter,
                not_like_filter_value,
            );
        };
        if let Some(not_like_filter_values) = filter
            .get(not_like_filter)
            .and_then(|v| v.as_array())
            .map(|s| {
                s.into_iter()
                    .map(|v| v.as_str())
                    .filter(|v| v.is_some())
                    .map(|v| v.unwrap())
                    .collect::<Vec<&str>>()
            })
        {
            builder.add_condition();
            count_builder.add_condition();
            builder.begin_multi_filter();
            count_builder.begin_multi_filter();
            for not_like_filter_value in not_like_filter_values {
                builder.add_multi_filter();
                count_builder.add_multi_filter();
                (builder, count_builder) = add_str_not_like_filter(
                    builder,
                    count_builder,
                    not_like_filter,
                    not_like_filter_value,
                );
            }
            builder.end_multi_filter();
            count_builder.end_multi_filter();
        };
    }

    return (builder, count_builder);
}

fn add_num_equal_filter<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    equal_filter: &str,
    equal_filter_value: i64,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    builder
        .builder
        .push(format!("{} = ", equal_filter))
        .push_bind(format!("{}", equal_filter_value));

    count_builder
        .builder
        .push(format!("{} = ", equal_filter))
        .push_bind(format!("{}", equal_filter_value));
    return (builder, count_builder);
}

fn add_num_equal_filters<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: &CardFilter,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    for equal_filter in NUM_EQUAL_FILTERS {
        if let Some(equal_filter_value) = filter
            .get(equal_filter)
            .and_then(|v| v.as_i64())
            .map(|s| s as i64)
        {
            builder.add_condition();
            count_builder.add_condition();
            (builder, count_builder) =
                add_num_equal_filter(builder, count_builder, equal_filter, equal_filter_value);
        };
        if let Some(equal_filter_values) =
            filter
                .get(equal_filter)
                .and_then(|v| v.as_array())
                .map(|s| {
                    s.into_iter()
                        .map(|v| v.as_i64())
                        .filter(|v| v.is_some())
                        .map(|v| v.unwrap())
                        .collect::<Vec<i64>>()
                })
        {
            builder.add_condition();
            count_builder.add_condition();
            builder.begin_multi_filter();
            count_builder.begin_multi_filter();
            for equal_filter_value in equal_filter_values {
                builder.add_multi_filter();
                count_builder.add_multi_filter();
                (builder, count_builder) =
                    add_num_equal_filter(builder, count_builder, equal_filter, equal_filter_value);
            }
            builder.end_multi_filter();
            count_builder.end_multi_filter();
        };
    }

    return (builder, count_builder);
}

fn add_str_equal_filter<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    equal_filter: &str,
    equal_filter_value: &str,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    builder.add_condition();
    builder
        .builder
        .push(format!("{} = ", equal_filter))
        .push_bind(format!("{}", equal_filter_value));

    count_builder.add_condition();
    count_builder
        .builder
        .push(format!("{} = ", equal_filter))
        .push_bind(format!("{}", equal_filter_value));
    return (builder, count_builder);
}

fn add_str_equal_filters<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: &CardFilter,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    for equal_filter in STR_EQUAL_FILTERS {
        if let Some(equal_filter_value) = filter
            .get(equal_filter)
            .and_then(|v| v.as_str())
            .map(|s| s as &str)
        {
            (builder, count_builder) =
                add_str_equal_filter(builder, count_builder, equal_filter, equal_filter_value);
        };
        if let Some(equal_filter_values) =
            filter
                .get(equal_filter)
                .and_then(|v| v.as_array())
                .map(|s| {
                    s.into_iter()
                        .map(|v| v.as_i64())
                        .filter(|v| v.is_some())
                        .map(|v| v.unwrap())
                        .collect::<Vec<i64>>()
                })
        {
            builder.begin_multi_filter();
            count_builder.begin_multi_filter();
            for equal_filter_value in equal_filter_values {
                builder.add_multi_filter();
                count_builder.add_multi_filter();
                (builder, count_builder) =
                    add_num_equal_filter(builder, count_builder, equal_filter, equal_filter_value);
            }
            builder.end_multi_filter();
            count_builder.end_multi_filter();
        };
    }

    return (builder, count_builder);
}

fn add_filters<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: Option<CardFilter>,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    let mut count = 10;
    let mut page = 0;
    if let Some(filter) = filter {
        let max_count = 100;
        if let Some(filter_count) = filter
            .get("count")
            .and_then(|v| v.as_i64())
            .map(|n| n as i64)
        {
            if filter_count <= max_count && filter_count > 0 {
                count = filter_count;
            }
        }

        if let Some(filter_page) = filter
            .get("page")
            .and_then(|v| v.as_i64())
            .map(|n| n as i64)
        {
            page = filter_page - 1;
        }

        (builder, count_builder) = add_str_like_filters(builder, count_builder, &filter);
        (builder, count_builder) = add_str_not_like_filters(builder, count_builder, &filter);
        (builder, count_builder) = add_num_equal_filters(builder, count_builder, &filter);
        (builder, count_builder) = add_str_equal_filters(builder, count_builder, &filter);
    }
    builder.builder.push(" group by name");
    builder.builder.push(format!(" limit {}", count));
    builder.builder.push(format!(" offset {}", page * count));
    let test = builder.builder.sql();
    println!("{:?}", test);
    let test = count_builder.builder.sql();
    println!("{:?}", test);
    return (builder, count_builder);
}

impl CardComplete {
    pub async fn list(
        conn: &mut PoolConnection<Sqlite>,
        filter: Option<CardFilter>,
    ) -> Result<CardCompleteResult, HttpError> {
        let builder = SqlQueryBuilder::new("select * from cards".to_string());
        let count_builder = SqlQueryBuilder::new("select count(1) as total from cards".to_string());

        let (mut builder, mut count_builder) = add_filters(builder, count_builder, filter);

        let query = builder.builder.build_query_as::<CardComplete>();
        let result: Result<Vec<CardComplete>, _> = query.fetch_all(&mut *conn).await;

        let count_query = count_builder.builder.build_query_as::<Count>();
        let count_result: Result<Count, _> = count_query.fetch_one(&mut *conn).await;
        match (result, count_result) {
            (Ok(cards), Ok(count_result)) => {
                return Ok(CardCompleteResult {
                    cards,
                    total: count_result.total,
                });
            }
            (Err(err), _) => {
                println!("err: {:?}", err);
                return Err(HttpError::InternalError(
                    "Failed to fetch cards".to_string(),
                ));
            }
            (_, Err(err)) => {
                println!("err: {:?}", err);
                return Err(HttpError::InternalError(
                    "Failed to fetch card count".to_string(),
                ));
            }
        };
    }
}

impl CardSimple {
    pub async fn list(
        conn: &mut PoolConnection<Sqlite>,
        filter: Option<CardFilter>,
    ) -> Result<CardSimpleResult, HttpError> {
        let query_start = "select
        name, manaValue, manaCost, keywords, text, 
        power, toughness, life, loyalty, colors, colorIdentity,
        type, types, subtypes, supertypes, borderColor, availability,
        flavorText, artist, leadershipSkills, printings
        from cards"
            .to_string();
        let builder = SqlQueryBuilder::new(query_start);
        let count_builder =
            SqlQueryBuilder::new("select count(distinct name) as total from cards".to_string());

        let (mut builder, mut count_builder) = add_filters(builder, count_builder, filter);

        let query = builder.builder.build_query_as::<CardSimple>();
        let result: Result<Vec<CardSimple>, _> = query.fetch_all(&mut *conn).await;

        let count_query = count_builder.builder.build_query_as::<Count>();
        let count_result: Result<Count, _> = count_query.fetch_one(&mut *conn).await;
        match (result, count_result) {
            (Ok(cards), Ok(count_result)) => {
                return Ok(CardSimpleResult {
                    cards,
                    total: count_result.total,
                });
            }
            (Err(err), _) => {
                println!("err: {:?}", err);
                return Err(HttpError::InternalError(
                    "Failed to fetch cards".to_string(),
                ));
            }
            (_, Err(err)) => {
                println!("err: {:?}", err);
                return Err(HttpError::InternalError(
                    "Failed to fetch card count".to_string(),
                ));
            }
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardSimpleResult {
    pub cards: Vec<CardSimple>,
    pub total: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardCompleteResult {
    pub cards: Vec<CardComplete>,
    pub total: i32,
}

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
#[sqlx(rename_all = "camelCase")]
pub struct Count {
    total: i32,
}

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
#[sqlx(rename_all = "camelCase")]
pub struct CardSimple {
    name: Option<String>,
    mana_value: Option<f64>,
    mana_cost: Option<String>,
    keywords: Option<String>,
    text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    life: Option<String>,
    loyalty: Option<String>,
    colors: Option<String>,
    color_identity: Option<String>,
    r#type: Option<String>,
    types: Option<String>,
    subtypes: Option<String>,
    supertypes: Option<String>,
    border_color: Option<String>,
    availability: Option<String>,
    flavor_text: Option<String>,
    artist: Option<String>,
    leadership_skills: Option<String>,
    printings: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
#[sqlx(rename_all = "camelCase")]
pub struct CardComplete {
    pub artist: Option<String>,
    pub artist_ids: Option<String>,
    pub ascii_name: Option<String>,
    pub attraction_lights: Option<String>,
    pub availability: Option<String>,
    pub booster_types: Option<String>,
    pub border_color: Option<String>,
    pub card_parts: Option<String>,
    pub color_identity: Option<String>,
    pub color_indicator: Option<String>,
    pub colors: Option<String>,
    pub defense: Option<String>,
    pub duel_deck: Option<String>,
    pub edhrec_rank: Option<i32>,
    pub edhrec_saltiness: Option<f64>,
    pub face_converted_mana_cost: Option<f64>,
    pub face_flavor_name: Option<String>,
    pub face_mana_value: Option<f64>,
    pub face_name: Option<String>,
    pub face_printed_name: Option<String>,
    pub finishes: Option<String>,
    pub flavor_name: Option<String>,
    pub flavor_text: Option<String>,
    pub frame_effects: Option<String>,
    pub frame_version: Option<String>,
    pub hand: Option<String>,
    pub has_alternative_deck_limit: Option<bool>,
    pub has_content_warning: Option<bool>,
    pub is_alternative: Option<bool>,
    pub is_full_art: Option<bool>,
    pub is_funny: Option<bool>,
    pub is_game_changer: Option<bool>,
    pub is_online_only: Option<bool>,
    pub is_oversized: Option<bool>,
    pub is_promo: Option<bool>,
    pub is_rebalanced: Option<bool>,
    pub is_reprint: Option<bool>,
    pub is_reserved: Option<bool>,
    pub is_story_spotlight: Option<bool>,
    pub is_textless: Option<bool>,
    pub is_timeshifted: Option<bool>,
    pub keywords: Option<String>,
    pub language: Option<String>,
    pub layout: Option<String>,
    pub leadership_skills: Option<String>,
    pub life: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub mana_value: Option<f64>,
    pub name: Option<String>,
    pub number: Option<String>,
    pub original_printings: Option<String>,
    pub original_release_date: Option<String>,
    pub original_text: Option<String>,
    pub other_face_ids: Option<String>,
    pub power: Option<String>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type: Option<String>,
    pub printings: Option<String>,
    pub produced_mana: Option<String>,
    pub promo_types: Option<String>,
    pub rarity: Option<String>,
    pub rebalanced_printings: Option<String>,
    pub related_cards: Option<String>,
    pub security_stamp: Option<String>,
    pub set_code: Option<String>,
    pub side: Option<String>,
    pub signature: Option<String>,
    pub sku_ids: Option<String>,
    pub source_products: Option<String>,
    pub subsets: Option<String>,
    pub subtypes: Option<String>,
    pub supertypes: Option<String>,
    pub text: Option<String>,
    pub toughness: Option<String>,
    pub r#type: Option<String>,
    pub types: Option<String>,
    pub uuid: Option<String>,
    pub variations: Option<String>,
    pub watermark: Option<String>,
}
