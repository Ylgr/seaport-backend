use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum, Clone)]
#[DieselTypePath = "crate::schema::sql_types::TokenType"]
pub enum TokenType {
    ERC20,
    ERC721,
    ERC1155,
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

#[derive(Deserialize)]
pub struct NewOrderConsideration {
    pub recipient: String,
    pub token_type: TokenType,
    pub token_address: Option<String>,
    pub amount: Option<String>,
    pub end_amount: Option<String>,
    pub identifier: Option<String>,
}


#[derive(Deserialize)]
pub struct NewOrderOffer {
    pub token_type: TokenType,
    pub token_address: Option<String>,
    pub amount: Option<String>,
    pub end_amount: Option<String>,
    pub identifier: Option<String>,
}

// new full order, it is one order with many considerations and offers
#[derive(Deserialize)]
pub struct NewFullOrder {
    pub signature: String,
    pub create_by: String,
    pub considerations: Vec<NewOrderConsideration>,
    pub offers: Vec<NewOrderOffer>,
}

// full order, it is one order with many considerations and offers
#[derive(Queryable, Serialize)]
pub struct FullOrder {
    pub id: Uuid,
    pub signature: String,
    pub create_by: String,
    pub considerations: Vec<Consideration>,
    pub offers: Vec<Offer>,
}

impl FullOrder {
    pub fn from_joined_tables(orders: Vec<(Order, Consideration, Offer)>) -> Vec<FullOrder> {
        let mut full_orders = Vec::new();

        for (order, consideration, offer) in orders {
            let id = order.id;
            let signature = order.signature;
            let create_by = order.create_by;

            let entry = full_orders
                .iter_mut()
                .find(|full_order: &&mut FullOrder| full_order.id == id);

            match entry {
                Some(full_order) => {
                    full_order.considerations.push(consideration);

                    full_order.offers.push(offer);
                }
                None => {
                    let mut considerations = Vec::new();
                    let mut offers = Vec::new();

                    considerations.push(consideration);

                    offers.push(offer);

                    full_orders.push(FullOrder {
                        id,
                        signature,
                        create_by,
                        considerations,
                        offers,
                    });
                }
            }
        }

        full_orders
    }
}