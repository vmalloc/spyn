pub(crate) fn run_shell(cmd: &mut std::process::Command) -> anyhow::Result<()> {
    let res = cmd.spawn()?.wait()?;
    if res.code().unwrap() != 0 {
        anyhow::bail!("Command failed");
    }
    Ok(())
}
