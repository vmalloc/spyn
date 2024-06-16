use anyhow::Context;
use itertools::Itertools;
use reqs::Requirements;
use smol_str::SmolStr;
use std::{
    io::{BufRead, BufReader},
    os::unix::process::CommandExt,
};
use tracing::instrument;
use tracing_subscriber::util::SubscriberInitExt;

mod reqs;
mod utils;
mod venv;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Opts {
    /// Dependency to add to the new environment
    #[clap(long = "dep", short = 'd')]
    deps: Vec<smol_str::SmolStr>,

    #[clap(long = "requirements-file", short = 'r')]
    req_files: Vec<smol_str::SmolStr>,

    /// Run in offline mode (i.e. avoid accessing the network)
    #[clap(long)]
    pub(crate) offline: bool,

    /// Launches IPython in the new environment
    #[clap(long)]
    ipython: bool,

    /// Launch Jupyter notebooks in the newly created environment
    #[clap(long)]
    notebook: bool,

    /// Specifies the version of Python to be used (accepts version numbers as well as full paths)
    #[clap(short = 'p', long)]
    python: Option<String>,

    cmd: Vec<smol_str::SmolStr>,
}

#[tracing::instrument(skip(opts))]
fn assemble_requirements(opts: &Opts) -> anyhow::Result<Requirements> {
    let _timer = crate::utils::Timer::new("assemble");

    let mut reqs = Requirements::new();

    for dep in opts.deps.iter() {
        reqs.add(dep.as_ref());
    }
    if opts.ipython {
        reqs.add("ipython");
    }

    if opts.notebook {
        reqs.add("jupyter");
        reqs.add("notebook");
    }

    if let Some(filename) = opts.cmd.first() {
        if let Ok(file) = std::fs::File::open(filename.as_str()) {
            reqs.parse_and_append(file)?;
        }
    }

    for reqfile in opts.req_files.iter() {
        reqs.extend(
            BufReader::new(std::fs::File::open(reqfile.as_str())?)
                .lines()
                .map_ok(SmolStr::from)
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    Ok(reqs)
}

#[tracing::instrument(skip(opts))]
fn prepare_venv(opts: &Opts) -> anyhow::Result<venv::Venv> {
    let _timer = crate::utils::Timer::new("prepare");
    let reqs = assemble_requirements(opts).context("Failed assembling requirements")?;

    let hash = reqs.hash(opts.python.as_deref());

    let root = homedir::get_my_home()
        .context("Failed getting home directory")?
        .ok_or_else(|| anyhow::format_err!("Failed locating home directory"))?
        .join(".spyn");

    let venv_path = root.join(hash);
    let returned = crate::venv::Venv::new(&venv_path, opts.python.as_ref());

    if !venv_path.exists() {
        let temp_root = root.join("tmp");
        if !temp_root.exists() {
            std::fs::create_dir_all(temp_root).context("Failed creating .spyn directory")?;
        }

        if let Err(e) = returned.prepare(opts, reqs) {
            tracing::error!("Error preparing virtualenv: {e:?}");
            if let Err(e) = returned.purge() {
                tracing::warn!("Failed removing virtualenv directory: {e:?}");
            }
            return Err(e);
        }
    } else {
        tracing::debug!(?venv_path, "Using existing virtualenv dir");
    }
    tracing::debug!("Virtualenv preparation complete");
    Ok(returned)
}

#[instrument]
fn init_logging() {
    let _timer = crate::utils::Timer::new("init-logging");
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .finish()
        .try_init()
        .unwrap();
}

fn main() -> anyhow::Result<()> {
    let timer = crate::utils::Timer::new("main");
    init_logging();

    ctrlc::set_handler(move || {
        tracing::debug!("Interrupted");
    })
    .context("Error setting up signal handler")?;

    let opts = Opts::parse();

    let venv = prepare_venv(&opts).context("Failed preparing virtual environment")?;

    let mut cmd = std::process::Command::new(venv.path().join("bin/python"));

    if opts.ipython {
        cmd.args(["-m", "IPython"]);
    } else if opts.notebook {
        // Jupyter requires PATH to point at the virtual environment in order to function properly
        let mut path = venv.path().join("bin").to_string_lossy().to_string();
        if let Ok(existing_path) = std::env::var("PATH") {
            path.push(':');
            path.push_str(&existing_path);
        }
        cmd.env("PATH", path);
        cmd.args(["-m", "jupyter", "notebook"]);
    }

    for arg in opts.cmd {
        cmd.arg(arg.as_str());
    }
    tracing::debug!(?cmd, "Running");
    drop(timer);
    Err(cmd.exec()).with_context(|| format!("Failed running process {cmd:?}"))
}
