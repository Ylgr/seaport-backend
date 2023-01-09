use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;
use uuid::Uuid;

#[derive(Debug, diesel_derive_enum::DbEnum, Clone, Deserialize)]
#[DieselTypePath = "crate::schema::sql_types::TokenType"]
pub enum TokenType {
    ERC20,
    ERC721,
    ERC1155
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: Uuid,
    pub signature: String,
    pub create_by: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub signature: String,
    pub create_by: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = considerations)]
pub struct NewConsideration {
    pub recipient: String,
    pub type_: TokenType,
    pub token_address: String,
    pub amount: String,
    pub end_amount: String,
    pub identifier: String,
}

#[derive(Insertable)]
#[diesel(table_name = offers)]
pub struct NewOffer {
    pub type_: TokenType,
    pub token_address: String,
    pub amount: String,
    pub end_amount: String,
    pub identifier: String,
}

#[derive(Deserialize)]
pub struct FullOrder {
    pub order: NewOrder,
    pub considerations: Vec<NewConsideration>,
}