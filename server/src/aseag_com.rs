use axum::{
    extract::Query, handler, response::{Html, IntoResponse}, routing::{get, post}, Router
};

pub struct BusRoute {
    id: u32,
    request_url: String,
}

pub fn aseag_com_routes() -> Router {
    Router::new()
        .route("/bus_route", get(request_bus))
}

pub async fn request_bus(Query(params): Query<BusRoute>) -> impl IntoResponse {

    let name = params.request_url;
    Html(format!("Params: {name}"))
}