use std::path::PathBuf;

use clap::Clap;
use once_cell::sync::Lazy;

pub fn init() -> Cmd {
    let cmd = Cmd::parse();
    cmd.opt.init_logger();
    cmd
}

#[derive(Clap, Debug)]
#[clap(version = VERSION_SHA.as_str())]
pub struct Cmd {
    #[clap(flatten)]
    pub opt: Opt,
    #[clap(subcommand)]
    pub sub: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// Initialize a new directory
    Init,
    /// Add or update a secret
    Encrypt {
        /// Key, must be all capital ASCII characters, digits, and underscores
        key: String,
        /// Value
        value: String,
    },
    /// Remove a secret
    Remove {
        /// Key, must be all capital ASCII characters, digits, and underscores
        key: String,
    },
    /// Print all of the secrets
    Print,
    /// Run a command with all of the secrets set as environment variables
    Exec {
        /// Command to run
        cmd: String,
        /// Command line arguments to pass to the command
        args: Vec<String>,
    },
}

static VERSION_SHA: Lazy<String> = Lazy::new(|| {
    format!(
        "{} (Git SHA1 {})",
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_GIT_SHA")
    )
});

/// Utility to store encrypted secrets in version trackable plain text files.
#[derive(Clap, Debug)]
pub struct Opt {
    /// Turn on verbose output
    #[clap(short, long, global = true)]
    pub verbose: bool,
    /// amber.yaml file location
    #[clap(long, default_value = "amber.yaml", global = true, env = "AMBER_YAML")]
    pub amber_yaml: PathBuf,
    /// Disable masking of secret values during exec
    #[clap(long, global = true)]
    pub unmasked: bool,
}

impl Opt {
    /// Initialize the logger based on command line settings
    pub fn init_logger(&self) {
        use env_logger::{Builder, Target};
        use log::LevelFilter::*;
        let mut builder = Builder::from_default_env();
        let level = if self.verbose { Debug } else { Info };
        builder.filter_module(env!("CARGO_CRATE_NAME"), level);
        builder.target(Target::Stderr).init();
    }
}
