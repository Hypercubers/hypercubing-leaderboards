use itertools::Itertools;

#[allow(dead_code)]
pub(crate) fn assert_send(_: impl Send) {}

pub fn concat_json_values(args: Vec<&handlebars::JsonValue>) -> String {
    args.into_iter()
        .map(|s| match s {
            handlebars::JsonValue::String(s) => s.clone(),
            other => other.to_string(),
        })
        .join("")
}

pub fn boolean_or_json_values(args: Vec<&handlebars::JsonValue>) -> bool {
    args.iter()
        .map(|arg| match arg {
            handlebars::JsonValue::Null => false,
            handlebars::JsonValue::Bool(b) => *b,
            handlebars::JsonValue::Number(number) => number
                .as_i128()
                .map(|n| n != 0)
                .or_else(|| number.as_f64().map(|n| n != 0.0))
                .unwrap_or(false),
            handlebars::JsonValue::String(s) => !s.is_empty(),
            handlebars::JsonValue::Array(values) => !values.is_empty(),
            handlebars::JsonValue::Object(_) => true,
        })
        .fold(false, |a, b| a | b)
}

pub fn html_render_time(time_cs: i32) -> String {
    let cs = time_cs % 100;
    let s = (time_cs / 100) % 60;
    let m = (time_cs / (100 * 60)) % 60;
    let h = (time_cs / (100 * 60 * 60)) % 24;
    let d = time_cs / (100 * 60 * 60 * 24);

    if d > 0 {
        format!("{d}<small>d</small> {h:0>2}<small>h</small> {m:0>2}<small>m</small> {s:0>2}.{cs:0>2}<small>s</small>")
    } else if h > 0 {
        format!("{h}<small>h</small> {m:0>2}<small>m</small> {s:0>2}.{cs:0>2}<small>s</small>")
    } else if m > 0 {
        format!("{m}<small>m</small> {s:0>2}.{cs:0>2}<small>s</small>")
    } else {
        format!("{s}.{cs:0>2}<small>s</small>")
    }
}

pub fn html_render_rank(rank: i32) -> String {
    let icon_html = match rank {
        1 => svg_icon("assets/gold-hypercube.svg"),
        2 => svg_icon("assets/silver-hypercube.svg"),
        3 => svg_icon("assets/bronze-hypercube.svg"),
        _ => String::new(),
    };
    format!("{icon_html} {rank}")
}

fn svg_icon(path: &str) -> String {
    format!(
        r#"
        <svg width="1rem" height="1rem">
            <image xlink:href="{path}" width="1rem" height="1rem"/>
        </svg>
        "#
    )
}

pub fn render_time(time_cs: i32) -> String {
    let cs = time_cs % 100;
    let s = (time_cs / 100) % 60;
    let m = (time_cs / (100 * 60)) % 60;
    let h = (time_cs / (100 * 60 * 60)) % 24;
    let d = time_cs / (100 * 60 * 60 * 24);

    if d > 0 {
        format!("{d}:{h:0>2}:{m:0>2}:{s:0>2}.{cs:0>2}")
    } else if h > 0 {
        format!("{h}:{m:0>2}:{s:0>2}.{cs:0>2}")
    } else if m > 0 {
        format!("{m}:{s:0>2}.{cs:0>2}")
    } else {
        format!("{s}.{cs:0>2}")
    }
}

pub fn md_escape(s: &str) -> String {
    let mut ret = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if c.is_ascii_punctuation() {
            ret.push('\\');
        }
        ret.push(c);
    }
    md_minimal_escape(&ret)
}

pub fn md_minimal_escape(s: &str) -> String {
    // Remove right-to-left override and other similar annoying symbols
    s.replace(
        [
            '\u{202E}', // RTL Override
            '\u{200F}', // RTL Mark
            '\u{202B}', // RTL Embedding
            '\u{200B}', // Zero-width space
            '\u{200D}', // Zero-width joiner
            '\u{200C}', // Zero-width non-joiner
        ],
        " ",
    )
    // Remove everyone and here mentions. Has to be put after ZWS replacement
    // because it utilises ZWS itself.
    .replace("@everyone", "@\u{200B}everyone")
    .replace("@here", "@\u{200B}here")
}
