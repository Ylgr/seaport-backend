use axum::{response::Html, routing::{get, post}, Router, Json, http::StatusCode, response::IntoResponse};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use self::models::*;
use diesel::prelude::*;
use seaport_server::*;
use std::borrow::Borrow;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/orders", get(get_orders))
        .route("/orders", post(create_orders));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// fn main() {
//     use self::schema::orders::dsl::*;
//
//     let connection = &mut establish_connection();
//     let results = orders
//         // .filter(published.eq(true))
//         // .limit(5)
//         .load::<Order>(connection)
//         .expect("Error loading posts");
//
//     println!("Displaying {} posts", results.len());
//     for order in results {
//         println!("{}", order.create_by);
//         println!("-----------\n");
//         println!("{}", order.signature);
//     }
// }

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}


async fn get_orders(
    Json(payload): Json<GetOrder>,
) -> impl IntoResponse {
    use self::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    let results = orders
        .filter(create_by.eq(payload.address))
        .load::<Order>(connection)
        .expect("Error loading order");
    // let results = orders
    //     .load::<Order>(connection)
    //     .expect("Error loading orders");
    (StatusCode::OK, Json(results))
}

async fn create_orders (
    Json(payload): Json<FullOrder>,
) -> impl IntoResponse {
    use self::schema::orders;
    use self::schema::considerations;
    let connection = &mut establish_connection();

    let order: Order = diesel::insert_into(orders::table)
        .values(&payload.order)
        .get_result(connection)
        .expect("Error saving order");

    let consideration = diesel::insert_into(considerations::table)
        .values(&payload.considerations)
        .except("Error saving considerations");

    (StatusCode::CREATED, Json(order))
}

#[derive(Deserialize)]
struct GetOrder {
    address: String,
}