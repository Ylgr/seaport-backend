use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;
use uuid::Uuid;
use diesel_derive_enum::DbEnum;

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum, Clone)]
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

#[derive(Queryable, Serialize)]
#[diesel(table_name = considerations)]
pub struct Consideration {
    pub id: Uuid,
    pub order_id: Uuid,
    pub recipient: String,
    pub token_type: TokenType,
    pub token_address: Option<String>,
    pub amount: Option<String>,
    pub end_amount: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = considerations)]
pub struct NewConsideration {
    pub order_id: Uuid,
    pub recipient: String,
    pub token_type: TokenType,
    pub token_address: Option<String>,
    pub amount: Option<String>,
    pub end_amount: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = offers)]
pub struct Offer {
    pub id: Uuid,
    pub order_id: Uuid,
    pub token_type: TokenType,
    pub token_address: Option<String>,
    pub amount: Option<String>,
    pub end_amount: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = offers)]
pub struct NewOffer {
    pub order_id: Uuid,
    pub token_type: TokenType,
    pub token_address: Option<String>,
    pub amount: Option<String>,
    pub end_amount: Option<String>,
    pub identifier: Option<String>,
}
