use crate::traits::RequestBody;
use crate::{api, html, static_files, AppState};

pub(crate) fn router() -> axum::Router<AppState> {
    use axum::routing::{get, post};
    use axum_embed::ServeEmbed;

    axum::Router::new()
        // Authentication
        .route("/sign-in", get(html::sign_in::SignInPage::as_handler_query))
        .route(
            "/sign-out",
            get(html::sign_out::SignOutPage::as_handler_query),
        )
        .route(
            "/request-otp-email",
            post(html::forms::email_sign_in::SignInEmailRequest::as_multipart_form_handler),
        )
        .route(
            "/request-otp-discord",
            post(html::forms::discord_sign_in::SignInDiscordRequest::as_multipart_form_handler),
        )
        .route(
            "/submit-otp",
            post(html::otp::SubmitOtpRequest::as_multipart_form_handler),
        )
        // Data pages
        .route(
            "/",
            get(html::puzzle_leaderboard::GlobalLeaderboard::as_handler_query),
        )
        .route(
            "/puzzle",
            get(html::puzzle_leaderboard::PuzzleLeaderboard::as_handler_query),
        )
        .route(
            "/solve-table/all",
            get(html::global_leaderboard::GlobalLeaderboardTable::as_handler_query),
        )
        .route(
            "/solve-table/puzzle",
            get(html::puzzle_leaderboard::PuzzleLeaderboardTable::as_handler_query),
        )
        .route(
            "/solve-table/user",
            get(html::user_page::SolverLeaderboardTable::as_handler_query),
        )
        .route(
            "/solve-table/user-submissions",
            get(html::submissions::SolverSubmissionsTable::as_handler_query),
        )
        .route(
            "/solve-table/pending-submissions",
            get(html::submissions::PendingSubmissionsTable::as_handler_query),
        )
        .route(
            "/solver",
            get(html::puzzle_leaderboard::SolverLeaderboard::as_handler_query),
        )
        .route("/solve", get(html::solve::SolvePage::as_handler_query))
        .route(
            "/submit-solve",
            get(html::forms::submit_solve::SubmitSolve::as_handler_query)
                .post(api::submit_solve::ManualSubmitSolveRequest::as_multipart_form_handler),
        )
        .route(
            "/edit-solve",
            get(html::forms::edit_solve::EditSolvePage::as_handler_query)
                .post(api::submit_solve::UpdateSolveRequest::as_multipart_form_handler),
        )
        .route(
            "/verify-solve",
            post(api::verify_solve::VerifySolveRequest::as_multipart_form_handler),
        )
        .route(
            "/my-submissions",
            get(html::submissions::MySubmissionsPage::as_handler_query),
        )
        .route(
            "/solver-submissions",
            get(html::submissions::SolverSubmissionsPage::as_handler_query),
        )
        .route(
            "/pending-submissions",
            get(html::submissions::PendingSubmissionsPage::as_handler_query),
        )
        // .route(
        //     "/settings",
        //     get(html::forms::Settings::as_handler_query)
        //         .post(api::profile::UpdateProfile::as_multipart_form_handler),
        // )
        // .route(
        //     "/update-solve-program",
        //     post(api::upload::UpdateSolveProgramVersionId::as_multipart_form_handler),
        // )
        .nest_service("/js", ServeEmbed::<static_files::JsFiles>::new())
        .nest_service("/css", ServeEmbed::<static_files::CssFiles>::new())
        .nest_service("/assets", ServeEmbed::<static_files::Assets>::new())
}
