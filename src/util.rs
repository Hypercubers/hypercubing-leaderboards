use futures::future::pending;
use futures::Future;

pub async fn hang_none<T>(fut: impl Future<Output = Option<T>>) -> T {
    match fut.await {
        Some(val) => val,
        None => pending().await,
    }
}

pub async fn wait_for_none<T>(
    fut: impl Future<Output = Option<T>>,
    duration: tokio::time::Duration,
) -> Option<T> {
    let request_happy = hang_none(fut);

    let sleep_fut = tokio::time::sleep(duration);
    tokio::pin!(sleep_fut);

    tokio::select! {
        u = request_happy => {
            Some(u)
        }
        _ = sleep_fut => {
            None
        }
    }
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
