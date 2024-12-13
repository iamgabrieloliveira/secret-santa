use crate::state::AppState;
use axum::extract::Path;
use axum::{extract::State, response::Html};

pub async fn reveal(
    State(state): State<std::sync::Arc<AppState>>,
    Path(id): Path<String>,
) -> Html<String> {
    let mut links = state.links.lock().unwrap();

    if let Some(recipient) = links.remove(&id) {
        Html(format!("You need to gift: {}", recipient))
    } else {
        Html("Invalid or expired link.".to_string())
    }
}
