use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::utils::run_shell;

pub(crate) struct Venv {
    path: PathBuf,
}

impl Venv {
    pub(crate) fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
        }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn prepare(&self, reqs: crate::reqs::Requirements) -> anyhow::Result<()> {
        let uv_path = which::which("uv")
            .context("Failed locating uv executable. Do you have uv installed?")?;
        tracing::debug!(?uv_path);
        run_shell(
            std::process::Command::new(&uv_path)
                .arg("venv")
                .arg(&self.path),
        )
        .context("Failed creating virtualenv via `uv`")?;

        if let Some(reqfile) = reqs.write_in(&self.path)? {
            run_shell(
                std::process::Command::new(&uv_path)
                    .current_dir(&self.path)
                    .env("VIRTUAL_ENV", &self.path)
                    .arg("pip")
                    .arg("install")
                    .arg("-r")
                    .arg(reqfile), //.arg(req_file.to_string_lossy()),
            )
            .context("Failed running installation in venv")?;
        }
        Ok(())
    }
}
