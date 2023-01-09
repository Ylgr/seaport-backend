use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;
use uuid::Uuid;

#[derive(Debug, diesel_derive_enum::DbEnum, Clone)]
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

#[derive(Insertable)]
#[diesel(table_name = considerations)]
pub struct Consideration<'a> {
    // pub order_id: Uuid,
    pub recipient: &'a str,
    pub type_: TokenType,
    pub token_address: &'a str,
    pub amount: &'a str,
    pub end_amount: &'a str,
    pub identifier: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = offers)]
pub struct NewOffer<'a> {
    // pub order_id: Uuid,
    pub type_: TokenType,
    pub token_address: &'a str,
    pub amount: &'a str,
    pub end_amount: &'a str,
    pub identifier: &'a str,
}