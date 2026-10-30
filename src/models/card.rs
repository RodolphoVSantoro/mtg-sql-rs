#![allow(clippy::needless_return, clippy::manual_let_else)]

use sqlx::FromRow;
use serde::{Deserialize, Serialize};

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
    pub total: i32,
}

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
#[sqlx(rename_all = "camelCase")]
pub struct CardSimple {
    pub name: Option<String>,
    pub mana_value: Option<f64>,
    pub mana_cost: Option<String>,
    pub keywords: Option<String>,
    pub text: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub life: Option<String>,
    pub loyalty: Option<String>,
    pub colors: Option<String>,
    pub color_identity: Option<String>,
    pub r#type: Option<String>,
    pub types: Option<String>,
    pub subtypes: Option<String>,
    pub supertypes: Option<String>,
    pub border_color: Option<String>,
    pub availability: Option<String>,
    pub flavor_text: Option<String>,
    pub artist: Option<String>,
    pub leadership_skills: Option<String>,
    pub printings: Option<String>,
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
