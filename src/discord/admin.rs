use crate::{AppResult, PoiseCtx};

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

#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn update(ctx: PoiseCtx<'_>) -> AppResult {
    tracing::trace!("Running update.sh ...");

    let update_output = std::process::Command::new("/bin/bash")
        .arg("update.sh")
        .spawn()?
        .wait_with_output()?;

    tracing::trace!(?update_output, "Completed update.sh");

    if !update_output.status.success() {
        ctx.reply(format!(
            "Error updating:\n\nStdout:\n```\n{}\n```\n\nStderr:\n```\n{}\n```",
            String::from_utf8_lossy(&update_output.stdout),
            String::from_utf8_lossy(&update_output.stderr)
        ))
        .await?;
        return Ok(());
    }

    ctx.reply("Successfully updated! Restarting ...").await?;

    ctx.data()
        .request_restart(format!(
            "Discord user {} requested self-update",
            ctx.author().name,
        ))
        .await;

    Ok(())
}

/// Block
#[poise::command(
    slash_command,
    required_permissions = "MANAGE_GUILD",
    subcommands("block_submissions", "block_all")
)]
pub async fn block(_ctx: PoiseCtx<'_>) -> AppResult {
    Ok(())
}

#[poise::command(slash_command, rename = "submissions")]
pub async fn block_submissions(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.reply("block submissions").await?;
    Ok(())
}

#[poise::command(slash_command, rename = "all")]
pub async fn block_all(ctx: PoiseCtx<'_>) -> AppResult {
    ctx.reply("block all").await?;
    Ok(())
}
