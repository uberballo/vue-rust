use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Router};
use http::header::CONTENT_TYPE;

use crate::controllers;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub async fn start(&self) {
        let routes = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route(
                "/levels",
                post(controllers::image_controller::get_color_levels),
            )
            .route(
                "/invert",
                post(controllers::image_controller::invert_colors),
            );

        let app = Router::new()
            .merge(routes)
            .layer(
                tower_http::cors::CorsLayer::new()
                    .allow_origin(tower_http::cors::Any)
                    .allow_headers([CONTENT_TYPE])
                    .allow_methods([axum::http::Method::GET, axum::http::Method::POST]),
            )
            .fallback(handler_404)
            .layer(tower_http::trace::TraceLayer::new_for_http());

        axum::Server::bind(&self.address.parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "invalid endpoint")
}
