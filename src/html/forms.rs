use crate::error::AppError;
use crate::AppState;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use itertools::Itertools;

pub async fn puzzle_options(state: &AppState) -> Result<String, AppError> {
    let mut puzzles = state.get_all_puzzles().await?;
    puzzles.sort_by_key(|p| p.name.clone());
    let puzzle_options = puzzles
        .into_iter()
        .map(|puzzle| format!(r#"<option value="{}">{}</option>"#, puzzle.id, puzzle.name))
        .join("");
    Ok(puzzle_options)
}

pub async fn program_version_options(state: &AppState) -> Result<String, AppError> {
    let mut program_versions = state.get_all_program_versions().await?;
    program_versions.sort_by_key(|p| (p.name()));
    let program_version_options = program_versions
        .into_iter()
        .map(|pv| format!(r#"<option value="{}">{}</option>"#, pv.id, pv.name()))
        .join("");
    Ok(program_version_options)
}

pub async fn upload_external(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let mut puzzles = state.get_all_puzzles().await?;
    puzzles.sort_by_key(|p| p.name.clone());

    let mut program_versions = state.get_all_program_versions().await?;
    program_versions.sort_by_key(|p| (p.name()));

    Ok(Html(
        crate::hbs!()
            .render_template(
                include_str!("../../html/upload-external.html"),
                &serde_json::json!({
                    "puzzles": puzzles,
                    "program_versions": program_versions,
                }),
            )
            .expect("render error"),
    ))
}

pub async fn sign_in() -> impl IntoResponse {
    Html(include_str!("../../html/sign-in.html"))
}

pub async fn update_profile() -> impl IntoResponse {
    Html(include_str!("../../html/update-profile.html"))
}
