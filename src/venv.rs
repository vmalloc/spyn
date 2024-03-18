use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::utils::run_shell;

pub(crate) struct Venv {
    path: PathBuf,
    python: Option<String>,
}

impl Venv {
    pub(crate) fn new(path: &Path, python: Option<&String>) -> Self {
        Self {
            path: path.to_owned(),
            python: python.cloned(),
        }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn prepare(
        &self,
        opts: &crate::Opts,
        temp_dir: tempfile::TempDir,
        reqs: crate::reqs::Requirements,
    ) -> anyhow::Result<()> {
        let uv_path = which::which("uv")
            .context("Failed locating uv executable. Do you have uv installed?")?;
        tracing::debug!(?uv_path);
        let mut cmd = std::process::Command::new(&uv_path);
        cmd.arg("venv");
        if opts.offline {
            cmd.arg("--offline");
        }
        if let Some(p) = &self.python {
            cmd.arg("--python").arg(p);
        }
        cmd.arg(temp_dir.path());
        run_shell(&mut cmd).context("Failed creating virtualenv via `uv`")?;

        if let Some(reqfile) = reqs.write_in(&temp_dir.path())? {
            let mut cmd = std::process::Command::new(&uv_path);
            cmd.current_dir(temp_dir.path())
                .env("VIRTUAL_ENV", temp_dir.path())
                .arg("pip")
                .arg("install");
            if opts.offline {
                cmd.arg("--offline");
            }
            cmd.arg("-r");
            cmd.arg(reqfile);
            run_shell(
                &mut cmd, //.arg(req_file.to_string_lossy()),
            )
            .with_context(|| {
                format!("Failed running installation in venv {:?}", &temp_dir.path())
            })?;
        }
        let temp_path = temp_dir.into_path();
        tracing::debug!(from=?temp_path, to=?self.path, "Moving virtualenv");
        std::fs::rename(&temp_path, &self.path)
            .context("Failed moving virtualenv to its final location")?;
        Ok(())
    }
}
