use std::sync::Arc;

use crate::state::AppState;
use axum::{extract::State, response::Html};

pub async fn list(State(state): State<Arc<AppState>>) -> Html<String> {
    let participants = state.participants.lock().unwrap().clone();

    if participants.is_empty() {
        return Html("No participants found. Add participants first.".to_string());
    }

    Html(format!("Participants: {:?}", participants))
}
