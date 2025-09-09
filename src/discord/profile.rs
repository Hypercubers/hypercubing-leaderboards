use std::fmt;

use crate::api::profile::*;
use crate::db::UserId;
use crate::{AppResult, PoiseCtx, PoiseCtxExt, RequestBody};

#[poise::command(slash_command)]
pub async fn update_user_discord_id(
    ctx: PoiseCtx<'_>,
    target_user_id: Option<UserId>,
    new_discord_account: Option<crate::sy::Member>,
) -> AppResult {
    let user = ctx.author_user().await?;
    let target_user_id = target_user_id.unwrap_or(user.id).0;

    let req = UpdateUserDiscordIdRequest {
        target_user_id,
        new_discord_id: new_discord_account.map(|m| m.user.id.get()),
    };
    let resp = req.request_via_discord(&ctx).await?;

    send_profile_update_reply(&ctx, "Discord iD", resp.new_discord_id).await
}

#[poise::command(slash_command)]
pub async fn update_user_email(
    ctx: PoiseCtx<'_>,
    target_user_id: Option<UserId>,
    new_email: Option<String>,
) -> AppResult {
    let user = ctx.author_user().await?;
    let target_user_id = target_user_id.unwrap_or(user.id).0;

    let req = UpdateUserEmailRequest {
        target_user_id,
        new_email,
    };
    let resp = req.request_via_discord(&ctx).await?;

    send_profile_update_reply(&ctx, "email", resp.new_email).await
}

#[poise::command(slash_command)]
pub async fn update_user_name(
    ctx: PoiseCtx<'_>,
    target_user_id: Option<UserId>,
    new_name: Option<String>,
) -> AppResult {
    let user = ctx.author_user().await?;
    let target_user_id = target_user_id.unwrap_or(user.id).0;

    let req = UpdateUserNameRequest {
        target_user_id,
        new_name,
    };
    let resp = req.request_via_discord(&ctx).await?;

    send_profile_update_reply(&ctx, "name", resp.new_name).await
}

async fn send_profile_update_reply<T: fmt::Display>(
    ctx: &PoiseCtx<'_>,
    property: &str,
    new_value: Option<T>,
) -> AppResult {
    let msg = match new_value {
        Some(v) => format!("User {property} is now **{v}**"),
        None => format!("User {property} is now empty"),
    };
    ctx.send(poise::CreateReply::default().ephemeral(true).content(msg))
        .await?;
    Ok(())
}
