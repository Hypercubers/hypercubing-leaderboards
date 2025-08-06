use std::borrow::Cow;

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use handlebars::Handlebars;

use crate::db::User;

lazy_static! {
    /// Handlebars templates.
    pub static ref HBS: handlebars::Handlebars<'static> =
        load_handlebars_templates().expect("error initializing Handlebars templates");
}

/// Loads handlebars templates from disk in debug mode or from the binary in
/// release mode.
fn load_handlebars_templates() -> Result<Handlebars<'static>, handlebars::TemplateError> {
    use chrono::{DateTime, Utc};
    use handlebars::handlebars_helper;

    let mut hbs = Handlebars::new();
    hbs.set_strict_mode(true);
    hbs.set_dev_mode(cfg!(debug_assertions));

    handlebars_helper!(render_time: |t: Option<i32>| t.map(crate::util::html_render_time));
    hbs.register_helper("render_time", Box::new(render_time));
    handlebars_helper!(render_rank: |n: i32| crate::util::html_render_rank(n));
    hbs.register_helper("render_rank", Box::new(render_rank));
    handlebars_helper!(date: |t: DateTime<Utc>| t.date_naive().to_string());
    hbs.register_helper("date", Box::new(date));
    handlebars_helper!(concat: |*args| crate::util::concat_json_values(args));
    hbs.register_helper("concat", Box::new(concat));
    handlebars_helper!(render_verified: |b: Option<bool>| crate::util::render_verified(b));
    hbs.register_helper("render_verified", Box::new(render_verified));

    hbs.register_embed_templates_with_extension::<HtmlTemplates>(".hbs")?; // .hbs

    Ok(hbs)
}

pub fn render_html_template(
    template_name: &str,
    active_user: &Option<User>,
    data: serde_json::Value,
) -> Response {
    match render_html_template_internal(template_name, active_user, data) {
        Ok(resp) => resp,
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, {
            let error_msg = format!("template error: {e}");
            let data = serde_json::json!({ "error_msg": error_msg });
            render_html_template_internal("error.html", active_user, data).unwrap_or_else(|e| {
                format!("double template error: {e}\n{error_msg}").into_response()
            })
        })
            .into_response(),
    }
    .into_response()
}

fn render_html_template_internal(
    template_name: &str,
    active_user: &Option<User>,
    mut data: serde_json::Value,
) -> Result<Response, handlebars::RenderError> {
    if let serde_json::Value::Object(m) = &mut data {
        m.insert(
            "active_user".to_string(),
            active_user
                .as_ref()
                .map(|u| u.to_header_json())
                .unwrap_or_default(),
        );
    }
    HBS.render(template_name, &data)
        .map(|s| Html(s).into_response())
}

#[derive(rust_embed::RustEmbed, Copy, Clone)]
#[folder = "./html"]
#[include = "*.hbs"]
pub struct HtmlTemplates;

#[derive(rust_embed::RustEmbed, Copy, Clone)]
#[folder = "./js"]
#[include = "*.js"]
pub struct JsFiles;

#[derive(rust_embed::RustEmbed, Copy, Clone)]
#[folder = "./css"]
#[include = "*.css"]
pub struct CssFiles;

#[derive(rust_embed::RustEmbed, Copy, Clone)]
#[folder = "./assets"]
pub struct Assets;

fn get_file_handler<E: rust_embed::RustEmbed, T: IntoResponse>(
    mime_type_constructor: fn(Cow<'static, [u8]>) -> T,
    file_path: &str,
) -> Result<T, impl IntoResponse> {
    match E::get(file_path) {
        Some(file) => Ok(mime_type_constructor(file.data)),
        None => {
            let type_name = std::any::type_name::<E>();
            let error_msg = format!("cannot find requested file {file_path} in {type_name}");
            tracing::error!(error_msg);
            Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg))
        }
    }
}
