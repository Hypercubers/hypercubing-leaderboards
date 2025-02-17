use std::borrow::Cow;

use axum::handler::Handler;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::response::{Css, JavaScript};
use handlebars::Handlebars;

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

    use crate::db::program::ProgramVersion;

    let mut hbs = Handlebars::new();
    hbs.set_strict_mode(true);
    hbs.set_dev_mode(cfg!(debug_assertions));

    handlebars_helper!(name_ProgramVersion: |p: ProgramVersion| p.name());
    hbs.register_helper("name_ProgramVersion", Box::new(name_ProgramVersion));
    handlebars_helper!(render_time: |t: i32| crate::util::html_render_time(t));
    hbs.register_helper("render_time", Box::new(render_time));
    handlebars_helper!(date: |t: DateTime<Utc>| t.date_naive().to_string());
    hbs.register_helper("date", Box::new(date));

    hbs.register_embed_templates_with_extension::<HtmlTemplates>(".hbs")?; // .hbs

    hbs.register_partial("layout", include_str!("../html/layout.html.hbs"))?;

    Ok(hbs)
}

#[derive(rust_embed::RustEmbed)]
#[folder = "./html"]
#[include = "*.hbs"]
pub struct HtmlTemplates;

#[derive(rust_embed::RustEmbed)]
#[folder = "./js"]
#[include = "*.js"]
pub struct JsFiles;
impl JsFiles {
    pub fn get_handler<S>(file_path: &'static str) -> impl Handler<((),), S> {
        move || async { get_file_handler::<JsFiles, _>(JavaScript, file_path) }
    }
}

#[derive(rust_embed::RustEmbed)]
#[folder = "./css"]
#[include = "*.css"]
pub struct CssFiles;
impl CssFiles {
    pub fn get_handler<S>(file_path: &'static str) -> impl Handler<((),), S> {
        move || async { get_file_handler::<CssFiles, _>(Css, file_path) }
    }
}

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
