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
    handlebars_helper!(render_time: |t: i32| crate::util::render_time(t));
    hbs.register_helper("render_time", Box::new(render_time));
    handlebars_helper!(date: |t: DateTime<Utc>| t.date_naive().to_string());
    hbs.register_helper("date", Box::new(date));

    #[cfg(not(feature = "embed"))]
    hbs.register_templates_directory("./html", Default::default())?; // .hbs
    #[cfg(feature = "embed")]
    {
        #[derive(rust_embed::RustEmbed)]
        #[folder = "./html"]
        #[include = "*.hbs"]
        struct HtmlTemplates;

        hbs.register_embed_templates_with_extension::<HtmlTemplates>(".hbs")?; // .hbs
    }

    hbs.register_partial("header", include_str!("../html/header.html.hbs"))?;

    Ok(hbs)
}
