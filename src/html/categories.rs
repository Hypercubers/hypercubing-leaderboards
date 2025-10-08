use axum::response::IntoResponse;

use crate::db::{Program, Puzzle, User, Variant};
use crate::traits::RequestBody;
use crate::{AppError, AppState};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct CategoriesPage {}

pub struct CategoriesPageResponse {
    user: Option<User>,
    puzzles: Vec<Puzzle>,
    variants: Vec<Variant>,
    programs: Vec<Program>,
}

impl RequestBody for CategoriesPage {
    type Response = CategoriesPageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if !user.as_ref().ok_or(AppError::NotLoggedIn)?.moderator {
            return Err(AppError::NotAuthorized);
        }

        let mut puzzles = state.get_all_puzzles().await?;
        let mut variants = state.get_all_variants().await?;
        let mut programs = state.get_all_programs().await?;

        puzzles.sort_by_key(|puzzle| puzzle.id);
        variants.sort_by_key(|variant| variant.id);
        programs.sort_by_key(|program| program.id);

        Ok(CategoriesPageResponse {
            user,
            puzzles,
            variants,
            programs,
        })
    }
}

impl IntoResponse for CategoriesPageResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template(
            "categories.html",
            &self.user,
            serde_json::json!({
                "puzzles": self.puzzles,
                "variants": self.variants,
                "programs": self.programs,
            }),
        )
    }
}
