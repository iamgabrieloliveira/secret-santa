use std::{collections::HashMap, sync::Arc};

use crate::state::AppState;
use axum::{extract::State, response::Html, Form};

pub async fn form() -> Html<&'static str> {
    Html(
        r#"
        <form action="/submit" method="post">
            <label>Enter participant names (comma-separated):</label>
            <input type="text" name="names" required>
            <button type="submit">Submit</button>
        </form>
    "#,
    )
}

pub async fn submit(
    State(state): State<Arc<AppState>>,
    Form(input): Form<HashMap<String, String>>,
) -> Html<String> {
    let names = input
        .get("names")
        .unwrap_or(&"".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    state.participants.lock().unwrap().extend(names);

    Html("Participants added! <a href='/generate-links'>Generate Links</a>".to_string())
}
