use axum::body::Body;
use axum::response::{IntoResponse, Response};

pub use crate::db::FullSolve;
use crate::db::{Program, Puzzle, SolveId, User};
use crate::traits::{Linkable, RequestBody};
use crate::{AppError, AppState};

#[derive(serde::Deserialize)]
pub struct SolvePage {
    id: SolveId,
}

pub struct SolvePageResponse {
    can_edit: bool,
    puzzles: Vec<Puzzle>,
    programs: Vec<Program>,
    title: String,
    title_html: String,
    verification_message: Option<String>,
    solve: FullSolve,
    user: Option<User>,
    youtube_embed_code: Option<String>,
    trusted_video_url: bool,
    show_speed: bool,
    show_fmc: bool,
    show_verification_status: bool,
}

impl RequestBody for SolvePage {
    type Response = SolvePageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let solve = state.get_solve(self.id).await?;

        if !solve.can_view_opt(user.as_ref()) {
            return Err(AppError::NotAuthorized);
        }

        let edit_auth = solve.can_edit_opt(user.as_ref());

        let show_speed = solve.can_view_speed(user.as_ref());
        let show_fmc = solve.can_view_fmc(user.as_ref());
        let show_verification_status = user
            .as_ref()
            .is_some_and(|u| u.moderator || u.id == solve.solver.id);

        let mut puzzles = state.get_all_puzzles().await?;
        puzzles.sort_by_key(|p| p.name.clone());

        let mut programs = state.get_all_programs().await?;
        programs.sort_by_key(|p| (p.material, p.name.clone()));

        let event = solve.primary_event();
        let mut title = event.name();
        let mut title_html = format!(
            r#"<strong><a href="{}">{}</a></strong>"#,
            event.relative_url(),
            event.name(),
        );
        if let Some(speed_cs) = solve.speed_cs.filter(|_| show_speed) {
            title += " in ";
            title += &crate::util::render_time(speed_cs);
            title_html += " in ";
            if solve.speed_verified == Some(false) {
                title_html += "<s>";
            }
            title_html += &format!("<strong>{}</strong>", crate::util::render_time(speed_cs));
            if solve.speed_verified == Some(false) {
                title_html += "</s>";
            }
        }
        if let Some(move_count) = solve.move_count.filter(|_| show_fmc) {
            if show_speed {
                title += " and ";
                title_html += " and ";
            } else {
                title += " in ";
                title_html += " in ";
            }
            title += &format!("{move_count} STM");
            if solve.fmc_verified == Some(false) {
                title_html += "<s>";
            }
            title_html += &format!("<strong>{move_count} <small>STM</small></strong>");
            if solve.fmc_verified == Some(false) {
                title_html += "</s>";
            }
        }
        title += &format!(" by {}", solve.solver.display_name());
        title_html += &format!(
            r#" by <strong><a href="{}">{}</a></strong>"#,
            solve.solver.relative_url(),
            solve.solver.display_name(),
        );

        // TODO: fix this for `t=` startcodes, and other params
        let youtube_embed_code = solve
            .video_url
            .as_ref()
            .and_then(|url| url.strip_prefix("https://youtu.be/"))
            .map(|s| s.to_string());

        let trusted_video_url = solve.speed_verified == Some(true)
            || solve
                .video_url
                .as_ref()
                .is_some_and(|url| crate::util::is_video_url_trusted(&url));

        // TODO: display non-youtube URLs as well

        let awaiting_speed_verification =
            solve.speed_cs.is_some() && solve.speed_verified.is_none();
        let awaiting_fmc_verification = solve.move_count.is_some() && solve.fmc_verified.is_none();
        let awaiting = r#"<span class="iconify" data-icon="mdi:timer"></span> Awaiting"#;
        // fizzbuzz
        let verification_message = match (awaiting_speed_verification, awaiting_fmc_verification) {
            (true, true) => Some(format!("{awaiting} speed + fewest-moves verification")),
            (true, false) => Some(format!("{awaiting} speed verification")),
            (false, true) => Some(format!("{awaiting} fewest-moves verification")),
            (false, false) => None,
        };

        Ok(SolvePageResponse {
            can_edit: edit_auth.is_some(),
            puzzles,
            programs,
            title,
            title_html,
            verification_message,
            solve,
            user,
            youtube_embed_code,
            trusted_video_url,
            show_speed,
            show_fmc,
            show_verification_status,
        })
    }
}

impl IntoResponse for SolvePageResponse {
    fn into_response(self) -> Response<Body> {
        crate::render_html_template(
            "solve.html",
            &self.user,
            serde_json::json!({
                "title": self.title,
                "title_html": self.title_html,
                "verification_message": self.verification_message,
                "solve": self.solve,
                "can_edit": self.can_edit,
                "solver_url": self.solve.solver.relative_url(),
                "solver_name": self.solve.solver.display_name(),
                "puzzle_url": self.solve.puzzle.relative_url(),
                "puzzle_name": self.solve.puzzle.name, // TODO: variant + program_material
                "puzzles": self.puzzles,
                "program": self.solve.program.name,
                "programs": self.programs,
                "youtube_embed_code": self.youtube_embed_code,
                "trusted_video_url": self.trusted_video_url,
                "show_speed": self.show_speed,
                "show_fmc": self.show_fmc,
                "show_verification_status": self.show_verification_status,
            }),
        )
    }
}
