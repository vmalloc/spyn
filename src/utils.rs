pub(crate) fn run_shell(cmd: &mut std::process::Command) -> anyhow::Result<()> {
    let res = cmd.spawn()?.wait()?;
    if res.code().unwrap() != 0 {
        anyhow::bail!("Command failed");
    }
    Ok(())
}

pub(crate) struct Timer {
    start: std::time::Instant,
    title: &'static str,
}

impl Timer {
    pub(crate) fn new(title: &'static str) -> Self {
        Self {
            start: std::time::Instant::now(),
            title,
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        tracing::debug!(phase=self.title, took=?std::time::Instant::now().duration_since(self.start))
    }
}
