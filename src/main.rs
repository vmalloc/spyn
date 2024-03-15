use reqs::Requirements;
use std::{os::unix::process::CommandExt, time::Instant};

mod reqs;
mod utils;
mod venv;

use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(long = "dep", short = 'd')]
    deps: Vec<String>,

    #[clap(long)]
    ipython: bool,

    cmd: Vec<String>,
}

fn assemble_requirements(opts: &Opts) -> anyhow::Result<Requirements> {
    let start = Instant::now();

    let mut reqs = Requirements::new();

    for dep in opts.deps.iter() {
        reqs.add(dep);
    }
    if opts.ipython {
        reqs.add("ipython");
    }

    if let Some(filename) = opts.cmd.first() {
        if let Ok(file) = std::fs::File::open(filename) {
            reqs.parse_and_append(file)?;
        }
    }

    tracing::debug!(
        duration = ?std::time::Instant::now().duration_since(start),
    );
    Ok(reqs)
}

fn prepare_venv(opts: &Opts) -> anyhow::Result<venv::Venv> {
    let reqs = assemble_requirements(opts)?;

    let path = tempdir::TempDir::new("spyn")?;

    let returned = crate::venv::Venv::new(path.path());
    returned.prepare(reqs)?;

    let _ = path.into_path();

    Ok(returned)
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let start = std::time::Instant::now();
    let opts = Opts::parse();

    let venv = prepare_venv(&opts)?;

    let mut cmd = std::process::Command::new(venv.path().join(format!(
        "bin/{}",
        if opts.ipython { "ipython" } else { "python" }
    )));

    for arg in opts.cmd {
        cmd.arg(arg);
    }

    tracing::debug!(
        "Virtual environment created in {:?}",
        std::time::Instant::now().duration_since(start)
    );
    Err(cmd.exec().into())
}
