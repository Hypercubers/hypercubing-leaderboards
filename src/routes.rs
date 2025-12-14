use crate::traits::RequestBody;
use crate::{AppState, api, html, static_files};

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
            "/sign-out-everywhere",
            get(html::sign_out::SignOutEverywherePage::as_handler_query),
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
        .route(
            "/submit-pkce",
            get(html::forms::confirm_pkce::ConfirmPkcePage::as_handler_query)
                .post(html::forms::confirm_pkce::ConfirmPkceRequest::as_multipart_form_handler),
        )
        .route(
            "/poll-pkce",
            post(api::pkce::LongPollPkceRequest::as_json_handler),
        )
        // Data pages
        .route(
            "/",
            get(html::leaderboards::global::GlobalLeaderboard::as_handler_query),
        )
        .route(
            "/puzzle",
            get(html::leaderboards::per_puzzle::PuzzleLeaderboard::as_handler_query),
        )
        .route(
            "/solve-table/all",
            get(html::leaderboards::global::GlobalLeaderboardTable::as_handler_query),
        )
        .route(
            "/solve-table/puzzle",
            get(html::leaderboards::per_puzzle::PuzzleLeaderboardTable::as_handler_query),
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
            get(html::user_page::SolverLeaderboard::as_handler_query),
        )
        .route("/solve", get(html::solve::SolvePage::as_handler_query))
        .route("/solve-file", get(html::solve::SolveFile::as_handler_query))
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
        .route(
            "/categories",
            get(html::categories::CategoriesPage::as_handler_query),
        )
        .route(
            "/audit-log/general",
            get(html::audit_log::GeneralAuditLogPage::as_handler_query),
        )
        .route(
            "/audit-log/solve",
            get(html::audit_log::SolveAuditLogPage::as_handler_query),
        )
        .route(
            "/audit-log/user",
            get(html::audit_log::UserAuditLogPage::as_handler_query),
        )
        .route("/users", get(html::users::UsersPage::as_handler_query))
        .route(
            "/settings",
            get(html::settings::SettingsPage::as_handler_query),
        )
        // API
        .route(
            "/self-info",
            get(api::auth::UserSelfInfoRequest::as_handler_query),
        )
        .route(
            "/update-name",
            post(api::edit_user::UpdateUserNameRequest::as_multipart_form_handler),
        )
        .route(
            "/verify-solve",
            post(api::verify_solve::VerifySolveRequest::as_multipart_form_handler),
        )
        .route(
            "/update-variant",
            post(api::categories::UpdateVariant::as_multipart_form_handler),
        )
        .route(
            "/update-program",
            post(api::categories::UpdateProgram::as_multipart_form_handler),
        )
        .route(
            "/update-puzzle",
            post(api::categories::UpdatePuzzle::as_multipart_form_handler),
        )
        .route(
            "/update-user",
            post(api::edit_user::UpdateUser::as_multipart_form_handler),
        )
        // Resources
        .nest_service("/js", ServeEmbed::<static_files::JsFiles>::new())
        .nest_service("/css", ServeEmbed::<static_files::CssFiles>::new())
        .nest_service("/assets", ServeEmbed::<static_files::Assets>::new())
}
