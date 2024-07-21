use futures::{future::pending, Future};
use serde::de::IntoDeserializer;
use serde::Deserialize;

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_ref().map(String::as_str);
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

pub fn on_as_true<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_ref().map(String::as_str);
    match opt {
        None | Some("") => Ok(false),
        Some(_s) => Ok(true),
    }
}

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
