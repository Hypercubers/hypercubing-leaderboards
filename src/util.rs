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
