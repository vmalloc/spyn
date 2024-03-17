use anyhow::Context;
use reqs::Requirements;
use std::os::unix::process::CommandExt;
use tracing::instrument;
use tracing_subscriber::util::SubscriberInitExt;

mod reqs;
mod utils;
mod venv;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Opts {
    #[clap(long = "dep", short = 'd')]
    deps: Vec<smol_str::SmolStr>,

    #[clap(long)]
    ipython: bool,

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

    if let Some(filename) = opts.cmd.first() {
        if let Ok(file) = std::fs::File::open(filename.as_str()) {
            reqs.parse_and_append(file)?;
        }
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
        if let Some(parent) = venv_path.parent() {
            std::fs::create_dir_all(parent).context("Failed creating directory")?;
        }
        returned
            .prepare(reqs)
            .context("Failed creating virtual environment")?;
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

    let opts = Opts::parse();

    let venv = prepare_venv(&opts).context("Failed preparing virtual environment")?;

    let mut cmd = std::process::Command::new(venv.path().join(format!(
        "bin/{}",
        if opts.ipython { "ipython" } else { "python" }
    )));

    for arg in opts.cmd {
        cmd.arg(arg.as_str());
    }
    drop(timer);
    Err(cmd.exec().into())
}
