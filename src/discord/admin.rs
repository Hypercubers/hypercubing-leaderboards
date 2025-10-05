use crate::{AppResult, PoiseCtx};

#[poise::command(slash_command)]
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

#[poise::command(slash_command)]
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

#[poise::command(slash_command)]
pub async fn update(ctx: PoiseCtx<'_>) -> AppResult {
    let update_output = std::process::Command::new("/bin/bash")
        .arg("update.sh")
        .spawn()?
        .wait_with_output()?;

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
