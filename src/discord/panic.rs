use std::sync::atomic::Ordering::Relaxed;

use crate::db::UserId;
use crate::{AppResult, PoiseCtx};

#[poise::command(
    slash_command,
    required_permissions = "MANAGE_GUILD",
    subcommands("block", "unblock", "logout", "logout_all")
)]
pub async fn panic(_ctx: PoiseCtx<'_>) -> AppResult {
    Ok(())
}

/// Invalidate all tokens for a specific user
#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn logout(ctx: PoiseCtx<'_>, target_user_id: UserId) -> AppResult {
    ctx.data()
        .invalidate_all_tokens_for_user(target_user_id)
        .await?;
    ctx.reply(format!(
        "Deleted all session tokens for user {target_user_id}",
    ))
    .await?;
    Ok(())
}

/// Invalidate ALL user tokens, logging out ALL users (including moderators)
#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn logout_all(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.data().invalidate_all_tokens_for_all_users().await?;
    ctx.reply("Deleted all session tokens for all users")
        .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    required_permissions = "MANAGE_GUILD",
    subcommands(
        "block_submissions",
        "block_user_actions",
        "block_moderator_actions",
        "block_logins",
        "block_all"
    )
)]
pub async fn block(_ctx: PoiseCtx<'_>) -> AppResult {
    Ok(())
}

/// Block all new logins
#[poise::command(
    slash_command,
    rename = "logins",
    required_permissions = "MANAGE_GUILD"
)]
pub async fn block_logins(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.data().block_logins.store(true, Relaxed);
    ctx.reply("Logins are now blocked. Use `/panic unblock` to unblock all actions.")
        .await?;
    Ok(())
}

/// Block all new solve submissions
#[poise::command(
    slash_command,
    rename = "submissions",
    required_permissions = "MANAGE_GUILD"
)]
pub async fn block_submissions(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.data().block_solve_submissions.store(true, Relaxed);
    ctx.reply("New submissions are now blocked. Use `/panic unblock` to unblock all actions.")
        .await?;
    Ok(())
}

/// Block all non-read actions by ordinary users
#[poise::command(slash_command, rename = "user", required_permissions = "MANAGE_GUILD")]
pub async fn block_user_actions(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.data().block_user_actions.store(true, Relaxed);
    ctx.reply(
        "Non-read user actions are now blocked. Use `/panic unblock` to unblock all actions.",
    )
    .await?;
    Ok(())
}

/// Block all non-read actions by moderators
#[poise::command(
    slash_command,
    rename = "moderator",
    required_permissions = "MANAGE_GUILD"
)]
pub async fn block_moderator_actions(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.data().block_moderator_actions.store(true, Relaxed);
    ctx.reply(
        "Non-read moderator actions are now blocked. Use `/panic unblock` to unblock all actions.",
    )
    .await?;
    Ok(())
}

/// Block all actions except simple reads
#[poise::command(slash_command, rename = "all", required_permissions = "MANAGE_GUILD")]
pub async fn block_all(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.data().block_solve_submissions.store(true, Relaxed);
    ctx.data().block_user_actions.store(true, Relaxed);
    ctx.data().block_moderator_actions.store(true, Relaxed);
    ctx.reply("All non-read actions are now blocked. Use `/panic unblock` to unblock all actions.")
        .await?;
    Ok(())
}

/// Unblocks all actions
#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn unblock(ctx: PoiseCtx<'_>) -> AppResult {
    let mut s = String::new();
    if ctx.data().block_logins.swap(false, Relaxed) {
        s += "Unblocked logins.\n";
    }
    if ctx.data().block_solve_submissions.swap(false, Relaxed) {
        s += "Unblocked new submissions.\n";
    }
    if ctx.data().block_user_actions.swap(false, Relaxed) {
        s += "Unblocked user actions.\n";
    }
    if ctx.data().block_moderator_actions.swap(false, Relaxed) {
        s += "Unblocked moderator actions.\n";
    }
    if s.is_empty() {
        s += "No actions are blocked.";
    }
    ctx.reply(s).await?;
    Ok(())
}
