use axum::response::{IntoResponse, Redirect, Response};
use axum_typed_multipart::TryFromMultipart;

use crate::db::{
    Program, ProgramData, ProgramId, Puzzle, PuzzleData, PuzzleId, User, Variant, VariantData,
    VariantId,
};
use crate::{AppError, AppState, RequestBody};

#[derive(TryFromMultipart)]
pub struct UpdateVariant {
    pub id: Option<i32>,
    pub name: String,
    pub abbr: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub material: bool,
    pub filters: bool,
    pub macros: bool,
    pub audit_log_comment: Option<String>,
}
impl RequestBody for UpdateVariant {
    type Response = UpdateCategoriesResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let editor = user.ok_or(AppError::NotLoggedIn)?;

        if let Some(id) = self.id {
            state
                .update_variant(
                    &editor,
                    Variant {
                        id: VariantId(id),
                        name: self.name,
                        prefix: self.prefix.unwrap_or_default(),
                        suffix: self.suffix.unwrap_or_default(),
                        abbr: self.abbr,
                        material_by_default: self.material,
                        primary_filters: self.filters,
                        primary_macros: self.macros,
                    },
                    &self.audit_log_comment.unwrap_or_default(),
                )
                .await?;
        } else {
            state
                .add_variant(
                    &editor,
                    VariantData {
                        name: self.name,
                        prefix: self.prefix.unwrap_or_default(),
                        suffix: self.suffix.unwrap_or_default(),
                        abbr: self.abbr,
                        material_by_default: self.material,
                        primary_filters: self.filters,
                        primary_macros: self.macros,
                    },
                )
                .await?;
        }

        Ok(UpdateCategoriesResponse)
    }
}

#[derive(TryFromMultipart)]
pub struct UpdateProgram {
    pub id: Option<i32>,
    pub name: String,
    pub abbr: String,
    pub material: bool,
    pub audit_log_comment: Option<String>,
}
impl RequestBody for UpdateProgram {
    type Response = UpdateCategoriesResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let editor = user.ok_or(AppError::NotLoggedIn)?;

        if let Some(id) = self.id {
            state
                .update_program(
                    &editor,
                    Program {
                        id: ProgramId(id),
                        name: self.name,
                        abbr: self.abbr,
                        material: self.material,
                    },
                    &self.audit_log_comment.unwrap_or_default(),
                )
                .await?;
        } else {
            state
                .add_program(
                    &editor,
                    ProgramData {
                        name: self.name,
                        abbr: self.abbr,
                        material: self.material,
                    },
                )
                .await?;
        }

        Ok(UpdateCategoriesResponse)
    }
}

#[derive(TryFromMultipart)]
pub struct UpdatePuzzle {
    pub id: Option<i32>,
    pub name: String,
    pub filters: bool,
    pub macros: bool,
    pub hsc_id: Option<String>,
    pub autoverifiable: bool,
    pub audit_log_comment: Option<String>,
}
impl RequestBody for UpdatePuzzle {
    type Response = UpdateCategoriesResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let editor = user.ok_or(AppError::NotLoggedIn)?;

        if let Some(id) = self.id {
            state
                .update_puzzle(
                    &editor,
                    Puzzle {
                        id: PuzzleId(id),
                        name: self.name,
                        primary_filters: self.filters,
                        primary_macros: self.macros,
                        hsc_id: self.hsc_id,
                        autoverifiable: self.autoverifiable,
                    },
                    &self.audit_log_comment.unwrap_or_default(),
                )
                .await?;
        } else {
            state
                .add_puzzle(
                    &editor,
                    PuzzleData {
                        name: self.name,
                        primary_filters: self.filters,
                        primary_macros: self.macros,
                        hsc_id: self.hsc_id,
                        autoverifiable: self.autoverifiable,
                    },
                )
                .await?;
        }

        Ok(UpdateCategoriesResponse)
    }
}

#[must_use]
#[derive(serde::Serialize)]
pub struct UpdateCategoriesResponse;
impl IntoResponse for UpdateCategoriesResponse {
    fn into_response(self) -> Response {
        Redirect::to("/categories").into_response()
    }
}
