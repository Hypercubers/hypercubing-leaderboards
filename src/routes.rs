use crate::traits::RequestBody;
use crate::{api, html, static_files, AppState};

pub(crate) fn router() -> axum::Router<AppState> {
    use axum::routing::{get, post};
    use axum_embed::ServeEmbed;

    axum::Router::new()
        /*.route(
            "/api/v1/auth/request-otp",
            post(api::auth::user_request_otp),
        )
        .route(
            "/api/v1/auth/request-token",
            post(api::auth::user_request_token),
        )
        .route(
            "/api/v1/upload-solve",
            post(api::upload::UploadSolve::as_handler_file),
        )
        .route(
            "/api/v1/upload-solve-external",
            post(api::upload::UploadSolveExternal::as_handler_file),
            //post(api::upload::UploadSolveExternal::show_all), // api endpoint for sign out
        )*/
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
        // .route("/api/v1/sign-in/discord", post(api::sign_in::SignInDiscord))
        // .route("/api/v1/auth-status", get(api::auth::AuthStatus))
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
        // .route(
        //     "/submit",
        //     get(html::forms::submit_solve::SubmitSolve::as_handler_query)
        //         .post(api::upload::ManualSubmitSolve::as_multipart_form_handler),
        // )
        .route("/sign-in", get(html::sign_in::SignInPage::as_handler_query))
        .route(
            "/sign-out",
            get(html::sign_out::SignOutPage::as_handler_query),
        )
        // .route(
        //     "/sign-in-discord",
        //     post(html::auth_discord::SignInDiscordForm::as_multipart_form_handler),
        // )
        // .route(
        //     "/request-otp",
        //     post(html::auth::RequestOtp::as_multipart_form_handler),
        // )
        // .route(
        //     "/sign-in-otp",
        //     get(html::sign_in::SignInOtpPage::as_handler_query)
        //         .post(html::auth::SignInOtp::as_multipart_form_handler),
        // )
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
        .route(
            "/update-solve-video-url",
            post(api::upload::UpdateSolveVideoUrl::as_multipart_form_handler),
        )
        .route(
            "/update-solve-speed-cs",
            post(api::upload::UpdateSolveSpeedCs::as_multipart_form_handler),
        )
        .route(
            "/update-solve-category",
            post(api::upload::UpdateSolveCategory::as_multipart_form_handler),
        )
        .route(
            "/update-solve-move-count",
            post(api::upload::UpdateSolveMoveCount::as_multipart_form_handler),
        )
        // .route(
        //     "/update-solve-program",
        //     post(api::upload::UpdateSolveProgramVersionId::as_multipart_form_handler),
        // )
        .nest_service("/js", ServeEmbed::<static_files::JsFiles>::new())
        .nest_service("/css", ServeEmbed::<static_files::CssFiles>::new())
        .nest_service("/assets", ServeEmbed::<static_files::Assets>::new())
}
