use axum::http::StatusCode;
use axum::{response::IntoResponse, Form};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(Form(form_data): Form<FormData>) -> impl IntoResponse {
    // println!("{:#?}", form_data);
    StatusCode::OK
}
