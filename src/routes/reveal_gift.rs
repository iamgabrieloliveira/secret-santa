use crate::state::AppState;
use axum::extract::Path;
use axum::{extract::State, response::Html};

pub async fn reveal(
    State(state): State<std::sync::Arc<AppState>>,
    Path(id): Path<String>,
) -> Html<String> {
    let mut links = state.links.lock().unwrap();

    if let Some(recipient) = links.remove(&id) {
        Html(format!(
            "Você tem que dar um presente para... {}",
            recipient
        ))
    } else {
        Html("Link invalido ou expirado.".to_string())
    }
}
