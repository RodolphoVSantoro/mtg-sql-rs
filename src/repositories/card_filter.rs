#![allow(clippy::needless_return, clippy::manual_let_else)]

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct EqualFilter {
    field: String,
    pub value: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LikeFilter {
    field: String,
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

fn get_num_filter(filter: &str) -> &str {
    return match filter {
        "manaValue" => "ca.manaValue",
        _ => "",
    };
}

fn get_str_filter(filter: &str) -> &str {
    return match filter {
        "name" => "ca.name",
        "text" => "ca.text",
        "flavorText" => "ca.flavorText",
        "keywords" => "ca.keywords",
        "manaCost" => "ca.manaCost",
        "life" => "ca.life",
        "power" => "ca.power",
        "toughness" => "ca.toughness",
        "loyalty" => "ca.loyalty",
        "colors" => "ca.colors",
        "colorIdentity" => "ca.colorIdentity",
        "type" => "ca.type",
        "types" => "ca.types",
        "subtypes" => "ca.subtypes",
        "supertypes" => "ca.supertypes",
        "borderColor" => "ca.borderColor",
        "artist" => "ca.artist",
        "leadershipSkills" => "ca.leadershipSkills",
        "printings" => "ca.printings",
        "scryfallId" => "ci.scryfallId",
        _ => "",
    };
}

impl LikeFilter {
    pub fn get_field(&self) -> &str {
        return get_str_filter(&self.field);
    }
    pub fn validate_field(&self) -> bool {
        return self.get_field() != "";
    }
}

impl EqualFilter {
    pub fn get_field(&self) -> &str {
        let str_filter = get_str_filter(&self.field);
        if !str_filter.is_empty() {
            return str_filter;
        }
        return get_num_filter(&self.field);
    }

    pub fn validate_field(&self) -> bool {
        return self.get_field() != "";
    }
}
