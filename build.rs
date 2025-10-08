#[allow(clippy::unwrap_used)]
fn main() {
    vergen_git2::Emitter::default()
        .add_instructions(
            &vergen_git2::Git2Builder::default()
                .sha(false)
                .build()
                .unwrap(),
        )
        .unwrap()
        .emit()
        .unwrap();
}
