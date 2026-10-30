#![allow(clippy::needless_return, clippy::manual_let_else)]

use serde::Deserialize;
use serde_json::Value;
use sqlx::{Sqlite, pool::PoolConnection};

use crate::helpers::query_builder::SqlQueryBuilder;
use crate::helpers::responses::HttpError;
use crate::models::card::{CardComplete, CardCompleteResult, CardSimple, CardSimpleResult, Count};

#[derive(Debug, Clone, Deserialize)]
pub struct EqualFilter {
    pub field: String,
    pub value: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LikeFilter {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotFilter {
    pub filter: CardFilter,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AndFilter {
    pub filters: Vec<CardFilter>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrFilter {
    pub filters: Vec<CardFilter>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "operation", rename_all = "lowercase")]
pub enum CardFilter {
    Equal(Box<EqualFilter>),
    Like(Box<LikeFilter>),
    Not(Box<NotFilter>),
    And(Box<AndFilter>),
    Or(Box<OrFilter>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct CardHttpQuery {
    pub page: Option<u32>,
    pub count: Option<u32>,
}

// .contains(&val)
const NUM_FILTERS: &[&str] = &["manaValue"];
const STR_FILTERS: &[&str] = &[
    "name",
    "text",
    "flavorText",
    "keywords",
    "manaCost",
    "power",
    "toughness",
    "life",
    "loyalty",
    "power",
    "toughness",
    "loyalty",
    "colors",
    "colorIdentity",
    "type",
    "types",
    "subtypes",
    "supertypes",
    "borderColor",
    "artist",
    "leadershipSkills",
    "printings"
];

fn validate_like_filter_type(filter: &LikeFilter) -> bool {
    return STR_FILTERS.contains(&filter.field.as_str());
}

fn validate_equal_filter_type(filter: &EqualFilter) -> bool {
    return NUM_FILTERS.contains(&filter.field.as_str())
        || STR_FILTERS.contains(&filter.field.as_str());
}

fn apply_equal_filter<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: EqualFilter,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    if !validate_equal_filter_type(&filter) {
        return (builder, count_builder);
    }
    builder.add_condition();
    builder
        .builder
        .push(format!("{} = ", &filter.field))
        .push_bind(format!("{}", &filter.value));

    count_builder.add_condition();
    count_builder
        .builder
        .push(format!("{} = ", &filter.field))
        .push_bind(format!("{}", &filter.value));
    return (builder, count_builder);
}

fn apply_like_filter<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: LikeFilter,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    if !validate_like_filter_type(&filter) {
        return (builder, count_builder);
    }
    builder.add_condition();
    builder
        .builder
        .push(format!("lower({}) like ", &filter.field))
        .push_bind(format!("%{}%", &filter.value));

    count_builder.add_condition();
    count_builder
        .builder
        .push(format!("lower({}) like", &filter.field))
        .push_bind(format!("%{}%", &filter.value));
    return (builder, count_builder);
}

fn apply_not_filter<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: NotFilter,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    builder.add_condition();
    builder.builder.push("not ");
    builder.begin_parentheses();

    count_builder.add_condition();
    count_builder.builder.push("not ");
    count_builder.begin_parentheses();

    (builder, count_builder) = add_filters(builder, count_builder, Some(filter.filter));

    builder.end_parentheses();
    count_builder.end_parentheses();

    return (builder, count_builder);
}

fn apply_and_filter<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: AndFilter,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    builder.add_condition();
    builder.begin_parentheses();

    count_builder.add_condition();
    count_builder.begin_parentheses();

    let mut has_filter = false;
    for inner_filter in filter.filters {
        if has_filter {
            builder.add_and();
            count_builder.add_and();
        }
        (builder, count_builder) = add_filters(builder, count_builder, Some(inner_filter));
        has_filter = true;
    }

    builder.end_parentheses();
    count_builder.end_parentheses();

    return (builder, count_builder);
}

fn apply_or_filter<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: OrFilter,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    builder.add_condition();
    builder.begin_parentheses();

    count_builder.add_condition();
    count_builder.begin_parentheses();

    let mut has_filter = false;
    for inner_filter in filter.filters {
        if has_filter {
            builder.add_or();
            count_builder.add_or();
        }
        (builder, count_builder) = add_filters(builder, count_builder, Some(inner_filter));
        has_filter = true;
    }

    builder.end_parentheses();
    count_builder.end_parentheses();

    return (builder, count_builder);
}

fn add_filters<'a>(
    builder: SqlQueryBuilder<'a>,
    count_builder: SqlQueryBuilder<'a>,
    filter: Option<CardFilter>,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    if let Some(filter) = filter {
        return match filter {
            CardFilter::Equal(equal_filter) => {
                apply_equal_filter(builder, count_builder, *equal_filter)
            }
            CardFilter::Like(like_filter) => {
                apply_like_filter(builder, count_builder, *like_filter)
            }
            CardFilter::Not(not_filter) => apply_not_filter(builder, count_builder, *not_filter),
            CardFilter::And(and_filter) => apply_and_filter(builder, count_builder, *and_filter),
            CardFilter::Or(or_filter) => apply_or_filter(builder, count_builder, *or_filter),
        };
    }
    return (builder, count_builder);
}

fn build_query<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: Option<CardFilter>,
    http_query: Option<CardHttpQuery>,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    let mut count = 10;
    let mut page = 0;
    if let Some(http_query) = http_query {
        let max_count = 100;
        if let Some(filter_count) = http_query.count {
            if filter_count <= max_count && filter_count > 0 {
                count = filter_count;
            }
        }

        if let Some(filter_page) = http_query.page {
            page = filter_page - 1;
        }
    }
    (builder, count_builder) = add_filters(builder, count_builder, filter);
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
        http_query: Option<CardHttpQuery>,
    ) -> Result<CardCompleteResult, HttpError> {
        let builder = SqlQueryBuilder::new("select * from cards".to_string());
        let count_builder = SqlQueryBuilder::new("select count(1) as total from cards".to_string());

        let (mut builder, mut count_builder) =
            build_query(builder, count_builder, filter, http_query);

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
        http_query: Option<CardHttpQuery>,
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

        let (mut builder, mut count_builder) =
            build_query(builder, count_builder, filter, http_query);

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
