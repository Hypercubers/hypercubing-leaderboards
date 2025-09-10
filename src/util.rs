use std::fmt;

use itertools::Itertools;
use rand::seq::IndexedRandom;
use rand::SeedableRng;

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

macro_rules! iconify_with_tooltip {
    ($icon:literal, $tooltip:literal) => {
        concat!(
            r#"
                <span class="tooltip" style="text-decoration: none;" data-tooltip=""#,
            $tooltip,
            r#"">
                    <span class="iconify" data-icon="mdi:"#,
            $icon,
            r#"">
                    </span>
                </span>
            "#
        )
    };
}

pub fn render_verified(is_verified: Option<bool>) -> &'static str {
    match is_verified {
        Some(true) => iconify_with_tooltip!("check", "Accepted"),
        Some(false) => iconify_with_tooltip!("close", "Rejected"),
        None => iconify_with_tooltip!("timer", "Awaiting verification"),
    }
}

/// Escapes Discord Markdown for safety and removing formatting.
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

/// Escapes Discord markdown for safety, but keeps formatting.
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

pub fn append_automated_moderator_note(
    moderator_notes: &mut String,
    new_log_msg: impl fmt::Display,
) {
    while !(moderator_notes.is_empty() || moderator_notes.ends_with("\n\n")) {
        moderator_notes.push('\n');
    }
    let now = chrono::Utc::now();
    *moderator_notes += &format!("[SYSTEM] [{now:?}] {new_log_msg}\n");
}

const URL_SCHEMES: &[&str] = &["http", "https"];
const TRUSTED_VIDEO_HOSTS: &[&str] = &["youtube.com", "youtu.be", "loom.com", "bilibili.com"];
pub fn is_video_url_trusted(url_str: &str) -> bool {
    url::Url::parse(url_str).is_ok_and(|url| {
        URL_SCHEMES.contains(&url.scheme()) && TRUSTED_VIDEO_HOSTS.contains(&url.authority())
    })
}

const BASE64_URL_SAFE: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
pub fn random_b64_string(len: usize) -> String {
    let mut rng = rand::rngs::StdRng::from_os_rng();
    (0..len)
        .map(|_| *BASE64_URL_SAFE.choose(&mut rng).unwrap() as char)
        .collect()
}

const DIGITS: &[u8; 10] = b"0123456789";
pub fn random_digits_string(len: usize) -> String {
    let mut rng = rand::rngs::StdRng::from_os_rng();
    (0..len)
        .map(|_| *DIGITS.choose(&mut rng).unwrap() as char)
        .collect()
}
