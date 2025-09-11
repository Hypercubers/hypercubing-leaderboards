use std::fmt;

use itertools::Itertools;
use serenity::all::Mentionable;

use crate::api::edit_user::*;
use crate::db::{EditAuthorization, User, UserId};
use crate::traits::Linkable;
use crate::util::md_escape;
use crate::{sy, AppResult, PoiseCtx, PoiseCtxExt, RequestBody};

#[poise::command(slash_command, subcommands("show", "set", "promote", "demote"))]
pub async fn user(_ctx: PoiseCtx<'_>) -> AppResult {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn show(ctx: PoiseCtx<'_>, target_user_id: Option<UserId>) -> AppResult {
    let user = ctx.author_user().await?;
    let target_user = match target_user_id {
        Some(id) => ctx.data().get_user(id).await?,
        None => user.clone(),
    };
    send_profile_info(&ctx, &target_user, user.edit_auth(target_user.id)).await
}

#[poise::command(slash_command, subcommands("set_discord", "set_email", "set_name"))]
pub async fn set(_ctx: PoiseCtx<'_>) -> AppResult {
    Ok(())
}

#[poise::command(slash_command, rename = "discord")]
pub async fn set_discord(
    ctx: PoiseCtx<'_>,
    target_user_id: Option<UserId>,
    new_discord_account: Option<crate::sy::Member>,
) -> AppResult {
    let user = ctx.author_user().await?;
    let target_user_id = target_user_id.unwrap_or(user.id);

    let req = UpdateUserDiscordIdRequest {
        target_user_id: target_user_id.0,
        new_discord_id: new_discord_account.map(|m| m.user.id.get()),
    };
    let resp = req.request_via_discord(&ctx).await?;

    send_profile_update_reply(&ctx, target_user_id, "Discord iD", resp.new_discord_id).await
}

#[poise::command(slash_command, rename = "email")]
pub async fn set_email(
    ctx: PoiseCtx<'_>,
    target_user_id: Option<UserId>,
    new_email: Option<String>,
) -> AppResult {
    let user = ctx.author_user().await?;
    let target_user_id = target_user_id.unwrap_or(user.id);

    let req = UpdateUserEmailRequest {
        target_user_id: target_user_id.0,
        new_email,
    };
    let resp = req.request_via_discord(&ctx).await?;

    send_profile_update_reply(&ctx, target_user_id, "email", resp.new_email).await
}

#[poise::command(slash_command, rename = "name")]
pub async fn set_name(
    ctx: PoiseCtx<'_>,
    target_user_id: Option<UserId>,
    new_name: Option<String>,
) -> AppResult {
    let user = ctx.author_user().await?;
    let target_user_id = target_user_id.unwrap_or(user.id);

    let req = UpdateUserNameRequest {
        target_user_id: target_user_id.0,
        new_name,
    };
    let resp = req.request_via_discord(&ctx).await?;

    send_profile_update_reply(&ctx, target_user_id, "name", resp.new_name).await
}

#[poise::command(slash_command)]
pub async fn promote(ctx: PoiseCtx<'_>, target_user_id: UserId) -> AppResult {
    update_user_is_moderator(&ctx, target_user_id, true).await
}

#[poise::command(slash_command)]
pub async fn demote(ctx: PoiseCtx<'_>, target_user_id: UserId) -> AppResult {
    update_user_is_moderator(&ctx, target_user_id, false).await
}

async fn update_user_is_moderator(
    ctx: &PoiseCtx<'_>,
    target_user_id: UserId,
    new_value: bool,
) -> AppResult {
    let user = ctx.author_user().await?;
    ctx.data()
        .update_user_is_moderator(&user, target_user_id, new_value)
        .await?;

    send_profile_update_reply(&ctx, target_user_id, "moderator flag", Some(new_value)).await
}

async fn send_profile_info(
    ctx: &PoiseCtx<'_>,
    user: &User,
    auth: Option<EditAuthorization>,
) -> AppResult {
    let mut embed = sy::CreateEmbed::new();

    let none = || "_none_".to_string();
    let md_escape_or_none =
        |opt: &Option<String>| opt.as_deref().map(md_escape).unwrap_or_else(none);

    let discord_account = user
        .discord_id
        .0
        .map(|id| sy::UserId::new(id).mention().to_string())
        .unwrap_or_else(none);

    let mut description = format!("### {}", user.to_public().md_link(false));
    description += &format!("\nLeaderboards ID: {}", user.id);
    description += &format!("\nName: {}", md_escape_or_none(&user.name));
    if auth.is_some() {
        let mut s = String::new();
        s += &format!("Discord: {discord_account}");
        s += &format!("\nEmail: {}", md_escape_or_none(&user.email));
        embed = embed.field("Contact", s, false);
    }
    if auth == Some(EditAuthorization::Moderator) {
        let flags_list = [
            user.dummy.then_some(":teddy_bear: dummy"),
            user.moderator.then_some(":shield: moderator"),
        ];
        let flags_str = flags_list.into_iter().filter_map(|x| x).join(", ");
        let field_name = match flags_str.is_empty() {
            true => "No flags",
            false => "Flags",
        };
        embed = embed.field(field_name, flags_str, false);

        let field_name = match user.moderator_notes.is_empty() {
            true => "No moderator notes",
            false => "Moderator notes",
        };
        embed = embed.field(field_name, &user.moderator_notes, false);

        if let Some(discord_id) = user.discord_id.0 {
            if let Ok(discord_user) = ctx.http().get_user(discord_id.into()).await {
                embed = embed.author(discord_user.into());
            }
        }
    }

    embed = embed.description(description);
    ctx.send(poise::CreateReply::default().ephemeral(true).embed(embed))
        .await?;
    Ok(())
}

async fn send_profile_update_reply<T: fmt::Display>(
    ctx: &PoiseCtx<'_>,
    target_user_id: UserId,
    property: &str,
    new_value: Option<T>,
) -> AppResult {
    let target_user = ctx.data().get_user(target_user_id).await?;
    let link = target_user.to_public().md_link(true);
    let msg = match new_value {
        Some(v) => format!("User {property} for {link} is now **{v}**"),
        None => format!("User {property} for {link} is now empty"),
    };
    ctx.send(poise::CreateReply::default().ephemeral(true).content(msg))
        .await?;
    Ok(())
}
