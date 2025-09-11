use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use handlebars::Handlebars;

use crate::db::User;
use crate::util::concat_json_values;
use crate::{AppError, AppResult};

lazy_static! {
    /// Handlebars templates.
    pub static ref HBS: handlebars::Handlebars<'static> =
        load_handlebars_templates().expect("error initializing Handlebars templates");
}

/// Loads handlebars templates from disk in debug mode or from the binary in
/// release mode.
#[allow(unused_parens)] // `handlebars_helper!` needs parens around multi-token types
fn load_handlebars_templates() -> Result<Handlebars<'static>, handlebars::TemplateError> {
    use chrono::{DateTime, Utc};
    use handlebars::handlebars_helper;

    let mut hbs = Handlebars::new();
    hbs.set_strict_mode(true);
    hbs.set_dev_mode(cfg!(debug_assertions));

    handlebars_helper!(render_time: |t: Option<i32>| t.map(crate::util::html_render_time));
    hbs.register_helper("render_time", Box::new(render_time));
    handlebars_helper!(date_from_datetime: |dt: DateTime<Utc>| dt.date_naive().to_string());
    hbs.register_helper("date_from_datetime", Box::new(date_from_datetime));
    handlebars_helper!(render_rank: |n: i32| crate::util::html_render_rank(n));
    hbs.register_helper("render_rank", Box::new(render_rank));
    handlebars_helper!(date: |t: DateTime<Utc>| t.date_naive().to_string());
    hbs.register_helper("date", Box::new(date));
    handlebars_helper!(concat: |*args| crate::util::concat_json_values(&args));
    hbs.register_helper("concat", Box::new(concat));
    handlebars_helper!(render_verified: |b: Option<bool>| crate::util::render_verified(b));
    hbs.register_helper("render_verified", Box::new(render_verified));
    handlebars_helper!(int_eq: |i1: i32, i2: i32| i1 == i2);
    hbs.register_helper("int_eq", Box::new(int_eq));
    handlebars_helper!(select_options: |options: Vec<serde_json::Value>, default_id: String, default_name: String, selected: (serde_json::Value)| {
        render_select_options(options, default_id, default_name, selected)
    });
    hbs.register_helper("select_options", Box::new(select_options));

    handlebars_helper!(cs_from_duration: |t: Option<i32>| t.map(|cs| cs % 100));
    hbs.register_helper("cs_from_duration", Box::new(cs_from_duration));
    handlebars_helper!(s_from_duration: |t: Option<i32>| t.map(|cs| cs / 100 % 60));
    hbs.register_helper("s_from_duration", Box::new(s_from_duration));
    handlebars_helper!(m_from_duration: |t: Option<i32>| t.map(|cs| cs / 100 / 60 % 60));
    hbs.register_helper("m_from_duration", Box::new(m_from_duration));
    handlebars_helper!(h_from_duration: |t: Option<i32>| t.map(|cs| cs / 100 / 60 / 60));
    hbs.register_helper("h_from_duration", Box::new(h_from_duration));

    handlebars_helper!(escape: |s: String| handlebars::html_escape(&s));
    hbs.register_helper("escape", Box::new(escape));

    hbs.register_embed_templates_with_extension::<HtmlTemplates>(".hbs")?;
    hbs.register_embed_templates_with_extension::<MessageTemplates>(".hbs")?;

    hbs.set_prevent_indent(true); // necessary for multiline form inputs

    Ok(hbs)
}

fn render_select_options(
    options: Vec<serde_json::Value>,
    default_id: String,
    default_name: String,
    selected: serde_json::Value,
) -> String {
    let mut ret = String::new();
    let mut any_selected = false;
    for opt in options {
        let id = opt.get("id").unwrap_or_default();
        let name = opt.get("name").unwrap_or_default();
        let is_selected = *id == selected;
        any_selected |= is_selected;

        let id_str = handlebars::html_escape(&concat_json_values(&[id]));
        let name_str = handlebars::html_escape(&concat_json_values(&[name]));
        let selected_str = if is_selected { " selected" } else { "" };
        ret.push_str(&format!(
            "<option value=\"{id_str}\"{selected_str}>{name_str}</option>\n",
        ));
    }
    if !(default_name.is_empty() && any_selected) {
        let selected_str = if any_selected { "" } else { " selected" };
        let disabled_hidden_str = if default_name.is_empty() {
            " disabled hidden"
        } else {
            ""
        };
        ret.insert_str(
            0,
            &format!("<option value=\"{default_id}\"{selected_str}{disabled_hidden_str}>{default_name}</option>\n"),
        );
    }
    ret
}

pub fn render_template(template_name: &str, data: &serde_json::Value) -> AppResult<String> {
    HBS.render(template_name, data)
        .map_err(|e| AppError::TemplateError(Box::new(e)))
}

pub fn render_html_template(
    template_name: &str,
    active_user: &Option<User>,
    data: serde_json::Value,
) -> Response {
    match render_html_template_internal(template_name, active_user, data) {
        Ok(resp) => resp,
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, {
            let error_msg = AppError::TemplateError(Box::new(e)).to_string();
            let data = serde_json::json!({ "error_msg": error_msg });
            render_html_template_internal("error.html", active_user, data).unwrap_or_else(|e| {
                AppError::DoubleTemplateError(Box::new(e), error_msg).into_response()
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
#[folder = "./assets"]
pub struct Assets;

#[derive(rust_embed::RustEmbed, Copy, Clone)]
#[folder = "./css"]
#[include = "*.css"]
pub struct CssFiles;

#[derive(rust_embed::RustEmbed, Copy, Clone)]
#[folder = "./html"]
#[include = "*.hbs"]
pub struct HtmlTemplates;

#[derive(rust_embed::RustEmbed, Copy, Clone)]
#[folder = "./js"]
#[include = "*.js"]
pub struct JsFiles;

#[derive(rust_embed::RustEmbed, Copy, Clone)]
#[folder = "./messages"]
#[include = "*.hbs"]
#[prefix = "messages/"]
pub struct MessageTemplates;
