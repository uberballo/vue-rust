use axum::{http::StatusCode, Json};
use axum_macros::debug_handler;

use crate::{
    services::image::{convert_and_get_levels, convert_and_invert_colors},
    types::{ColorLevels, UploadData},
};

// TODO User may send .png file which is not supported. Handle it properly.
#[debug_handler]
pub async fn get_color_levels(
    Json(payload): Json<UploadData>,
) -> (StatusCode, Result<Json<ColorLevels>, String>) {
    let result = convert_and_get_levels(payload.data);
    match result {
        Ok(levels) => (StatusCode::OK, Ok(Json(levels))),
        Err(e) => (StatusCode::BAD_REQUEST, Err(e.to_string())),
    }
}

#[debug_handler]
pub async fn invert_colors(
    Json(payload): Json<UploadData>,
) -> (StatusCode, Result<String, String>) {
    let result = convert_and_invert_colors(payload.data);
    match result {
        Ok(data_url) => (StatusCode::OK, Ok(data_url)),
        Err(e) => (StatusCode::BAD_REQUEST, Err(e.to_string())),
    }
}
