use crate::{
    db::SolveId,
    traits::{Linkable, PoiseCtx, PoiseCtxExt},
    AppResult,
};

#[poise::command(slash_command, subcommands("accept_speed", "accept_fmc"))]
pub async fn accept(_ctx: PoiseCtx<'_>) -> AppResult {
    Ok(())
}
/// Accept a speed submission
#[poise::command(slash_command, rename = "speed")]
async fn accept_speed(ctx: PoiseCtx<'_>, solve_id: SolveId) -> AppResult {
    speed_verify(ctx, solve_id, Some(true), "Accepted").await
}
/// Accept an FMC submission
#[poise::command(slash_command, rename = "fmc")]
async fn accept_fmc(ctx: PoiseCtx<'_>, solve_id: SolveId) -> AppResult {
    fmc_verify(ctx, solve_id, Some(true), "Accepted").await
}

#[poise::command(slash_command, subcommands("reject_speed", "reject_fmc"))]
pub async fn reject(_ctx: PoiseCtx<'_>) -> AppResult {
    Ok(())
}
/// Reject a speed submission
#[poise::command(slash_command, rename = "speed")]
async fn reject_speed(ctx: PoiseCtx<'_>, solve_id: SolveId) -> AppResult {
    speed_verify(ctx, solve_id, Some(false), "Rejected").await
}
/// Reject an FMC submission
#[poise::command(slash_command, rename = "fmc")]
async fn reject_fmc(ctx: PoiseCtx<'_>, solve_id: SolveId) -> AppResult {
    fmc_verify(ctx, solve_id, Some(false), "Rejected").await
}

#[poise::command(slash_command, subcommands("unverify_speed", "unverify_fmc"))]
pub async fn unverify(_ctx: PoiseCtx<'_>) -> AppResult {
    Ok(())
}
/// Unverify a speed submission
#[poise::command(slash_command, rename = "speed")]
async fn unverify_speed(ctx: PoiseCtx<'_>, solve_id: SolveId) -> AppResult {
    speed_verify(ctx, solve_id, None, "Unverified").await
}
/// Unverify an FMC submission
#[poise::command(slash_command, rename = "fmc")]
async fn unverify_fmc(ctx: PoiseCtx<'_>, solve_id: SolveId) -> AppResult {
    fmc_verify(ctx, solve_id, None, "Unverified").await
}

async fn speed_verify(
    ctx: PoiseCtx<'_>,
    solve_id: SolveId,
    verify: Option<bool>,
    verbed: &str,
) -> AppResult {
    let editor = ctx.author_user().await?;
    let state = ctx.data();
    let solve = state.get_solve(solve_id).await?;
    state.verify_speed(&editor, solve_id, verify).await?;
    ctx.say(format!(
        "{verbed} speed {} by {}",
        solve_id.md_link(false),
        solve.solver.md_link(false)
    ))
    .await?;
    Ok(())
}
async fn fmc_verify(
    ctx: PoiseCtx<'_>,
    solve_id: SolveId,
    verify: Option<bool>,
    verbed: &str,
) -> AppResult {
    let editor = ctx.author_user().await?;
    let state = ctx.data();
    let solve = state.get_solve(solve_id).await?;
    state.verify_fmc(&editor, solve_id, verify).await?;
    ctx.say(format!(
        "{verbed} fewest-moves {} by {}",
        solve_id.md_link(false),
        solve.solver.md_link(false),
    ))
    .await?;
    Ok(())
}
