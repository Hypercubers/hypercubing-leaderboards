use crate::{AppResult, PoiseCtx};

/// Display leaderboards version info
#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn version(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.reply(format!(
        "{} {}",
        env!("CARGO_PKG_NAME"),
        env!("VERGEN_GIT_SHA"),
    ))
    .await?;
    Ok(())
}

/// Shut down the leaderboards
#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn shutdown(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.reply("Shutting down ...").await?;

    ctx.data()
        .request_shutdown(format!(
            "Discord user {} requested shutdown",
            ctx.author().name,
        ))
        .await;

    Ok(())
}

/// Restart the leaderboards
#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn restart(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.reply("Restarting ...").await?;

    ctx.data()
        .request_restart(format!(
            "Discord user {} requested restart",
            ctx.author().name,
        ))
        .await;

    Ok(())
}

/// Update and restart the leaderboards
#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn update(ctx: PoiseCtx<'_>) -> AppResult {
    let reply = ctx.reply("Updating ...").await?;

    tracing::trace!("Downloading latest version ...");

    let update_output = std::process::Command::new("/bin/bash")
        .arg("update.sh")
        .spawn()?
        .wait_with_output()?;

    tracing::trace!(?update_output, "Completed update.sh");

    if !update_output.status.success() {
        let content = format!(
            "Error updating:\n\nStdout:\n```\n{}\n```\n\nStderr:\n```\n{}\n```",
            String::from_utf8_lossy(&update_output.stdout),
            String::from_utf8_lossy(&update_output.stderr),
        );
        reply
            .edit(ctx, poise::CreateReply::default().content(content))
            .await?;
        return Ok(());
    }

    let content = "Successfully updated! Restarting ...";
    reply
        .edit(ctx, poise::CreateReply::default().content(content))
        .await?;

    ctx.data()
        .request_restart(format!(
            "Discord user {} requested self-update",
            ctx.author().name,
        ))
        .await;

    Ok(())
}

#[poise::command(
    slash_command,
    required_permissions = "MANAGE_GUILD",
    subcommands("block_submissions", "block_all")
)]
pub async fn block(_ctx: PoiseCtx<'_>) -> AppResult {
    Ok(())
}

/// Block all new submissions
#[poise::command(
    slash_command,
    rename = "submissions",
    required_permissions = "MANAGE_GUILD"
)]
pub async fn block_submissions(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.reply("block submissions").await?;
    Ok(())
}

/// Block all actions except simple reads
#[poise::command(slash_command, rename = "all", required_permissions = "MANAGE_GUILD")]
pub async fn block_all(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.reply("block all").await?;
    Ok(())
}
