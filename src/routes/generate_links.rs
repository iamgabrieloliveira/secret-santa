use std::sync::Arc;

use crate::state::AppState;
use crate::utils::shuffle_and_generate_links;
use axum::{extract::State, response::Html};

pub async fn generate(State(state): State<Arc<AppState>>) -> Html<String> {
    let participants = state.participants.lock().unwrap().clone();
    if participants.is_empty() {
        return Html("No participants found. Add participants first.".to_string());
    }

    let mut links = state.links.lock().unwrap();
    *links = shuffle_and_generate_links(&participants);

    let link_list = links
        .iter()
        .map(|(id, _)| format!("<a href='/reveal/{}'>Link for participant</a>", id))
        .collect::<Vec<_>>()
        .join("<br>");

    Html(format!("Links generated:<br>{}", link_list))
}
