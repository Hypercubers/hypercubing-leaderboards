use crate::error::AppError;
use crate::AppState;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use itertools::Itertools;

pub async fn upload_external(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    println!("hewwo");
    let mut puzzles = state.get_all_puzzles().await?;
    puzzles.sort_by_key(|p| p.name.clone());
    let puzzle_options = puzzles
        .into_iter()
        .map(|puzzle| format!(r#"<option value="{}">{}</option>"#, puzzle.id, puzzle.name))
        .join("");

    let mut program_versions = state.get_all_program_versions().await?;
    program_versions.sort_by_key(|p| (p.name()));
    let program_version_options = program_versions
        .into_iter()
        .map(|pv| format!(r#"<option value="{}">{}</option>"#, pv.id, pv.name()))
        .join("");
    println!("hewwoooo");
    Ok(Html(format!(
        include_str!("../../html/upload-external.html"),
        puzzle_options = puzzle_options,
        program_version_options = program_version_options
    )))
}

pub async fn sign_in() -> impl IntoResponse {
    Html(include_str!("../../html/sign-in.html"))
}

pub async fn update_profile() -> impl IntoResponse {
    Html(include_str!("../../html/update-profile.html"))
}
