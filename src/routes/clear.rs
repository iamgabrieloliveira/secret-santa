use std::sync::Arc;

use axum::{extract::State, response::Html};

use crate::state::AppState;

pub async fn clear(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut participants = state.participants.lock().unwrap();
    let count = participants.len();

    participants.clear();

    return Html(format!("Participants removed: {}", count));
}
