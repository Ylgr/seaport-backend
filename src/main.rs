use axum::{response::Html, routing::{get, post}, Router, Json, http::StatusCode, response::IntoResponse};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use seaport_server::*;
use diesel::query_dsl::RunQueryDsl;
#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/orders", get(get_orders));
        // .route("/orders", post(create_orders));

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
    use crate::schema::orders::dsl;

    let connection = &mut establish_connection();
    let results = dsl::orders.load::<Order>(connection)
        .expect("Error loading order");
    // let results = orders
    //     .load::<Order>(connection)
    //     .expect("Error loading orders");
    (StatusCode::CREATED, Json({}))
}


#[derive(Deserialize)]
struct GetOrder {
    address: String,
}

    #[derive(Serialize)]
struct Order {
    create_by: String,
    signature: String,
}