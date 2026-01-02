use crate::{AppResult, PoiseCtx};

/// Display leaderboards version info
#[poise::command(slash_command)]
pub async fn version(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.reply(format!(
        "{} {}",
        env!("CARGO_PKG_NAME"),
        env!("VERGEN_GIT_SHA"),
    ))
    .await?;
    Ok(())
}

/// Shut down the leaderboards (IRREVERSIBLE WITHOUT SSH ACCESS)
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

    tracing::trace!("Downloading latest version of leaderboards ...");

    let update_output = async_process::Command::new("/bin/bash")
        .arg("update.sh")
        .output()
        .await?;

    tracing::trace!(?update_output, "Completed update.sh");

    if !update_output.status.success() {
        let content = format!(
            "Error updating leaderboards:\n\nStdout:\n```\n{}\n```\n\nStderr:\n```\n{}\n```",
            String::from_utf8_lossy(&update_output.stdout),
            String::from_utf8_lossy(&update_output.stderr),
        );
        reply
            .edit(ctx, poise::CreateReply::default().content(content))
            .await?;
        return Ok(());
    }

    tracing::trace!("Downloading latest version of Hyperspeedcube ...");

    let update_output = async_process::Command::new("/bin/bash")
        .arg("update-hsc.sh")
        .output()
        .await?;

    tracing::trace!(?update_output, "Completed update-hsc.sh");

    if !update_output.status.success() {
        let content = format!(
            "Successfully updated leaderboards.\nError updating HSC:\n\nStdout:\n```\n{}\n```\n\nStderr:\n```\n{}\n```",
            String::from_utf8_lossy(&update_output.stdout),
            String::from_utf8_lossy(&update_output.stderr),
        );
        reply
            .edit(ctx, poise::CreateReply::default().content(content))
            .await?;
        return Ok(());
    }

    let content = "Successfully updated leaderboards and HSC! Restarting ...";
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
