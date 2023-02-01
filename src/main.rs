use axum::{response::Html, routing::{get, post}, Router, Json, http::StatusCode, response::IntoResponse};
use std::net::SocketAddr;
use serde::Deserialize;
use self::models::*;
use diesel::prelude::*;
use seaport_server::*;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/orders", get(get_orders))
        .route("/order", post(create_full_order));
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}


async fn get_orders(
    Json(payload): Json<GetOrder>,
) -> impl IntoResponse {
    use self::schema::orders::dsl::*;
    use self::schema::considerations::dsl::*;
    use self::schema::offers::dsl::*;

    let connection = &mut establish_connection();
    let results = match payload.address {
        Some(address) => orders
            .inner_join(considerations)
            .inner_join(offers)
            .filter(create_by.eq(address))
            .load::<(Order, Consideration, Offer)>(connection)
            .expect("Error loading orders"),
        None => orders
            .inner_join(considerations)
            .inner_join(offers)
            .load::<(Order, Consideration, Offer)>(connection)
            .expect("Error loading orders"),
    };
    (StatusCode::OK, Json(
        FullOrder::from_joined_tables(results)
    ))
}

// apt create full order return a full order include its considerations and offers
async fn create_full_order(
    Json(payload): Json<NewFullOrder>,
) -> impl IntoResponse {
    use self::schema::orders::dsl::*;
    use self::schema::considerations::dsl::*;
    use self::schema::offers::dsl::*;

    let connection = &mut establish_connection();
    let new_order = NewOrder {
        signature: payload.signature,
        create_by: payload.create_by,
    };
    let order_result = diesel::insert_into(orders)
        .values(&new_order)
        .get_result::<Order>(connection)
        .expect("Error saving new order");

    let mut new_considerations = vec![];
    for consideration in payload.considerations {
        new_considerations.push(NewConsideration {
            order_id: order_result.id,
            recipient: consideration.recipient,
            token_type: consideration.token_type,
            token_address: consideration.token_address,
            amount: consideration.amount,
            end_amount: consideration.end_amount,
            identifier: consideration.identifier,
        });
    }
    let considerations_result = diesel::insert_into(considerations)
        .values(&new_considerations)
        .get_results::<Consideration>(connection)
        .expect("Error saving new consideration");

    let mut new_offers = vec![];
    for offer in payload.offers {
        new_offers.push(NewOffer {
            order_id: order_result.id,
            token_type: offer.token_type,
            token_address: offer.token_address,
            amount: offer.amount,
            end_amount: offer.end_amount,
            identifier: offer.identifier,
        });
    }
    let offers_result = diesel::insert_into(offers)
        .values(&new_offers)
        .get_results::<Offer>(connection)
        .expect("Error saving new offer");

    let result = FullOrder {
        id: order_result.id,
        signature: order_result.signature,
        create_by: order_result.create_by,
        considerations: considerations_result,
        offers: offers_result,
    };

    (StatusCode::OK, Json(result))
}

#[derive(Deserialize)]
struct GetOrder {
    address: Option<String>,
}