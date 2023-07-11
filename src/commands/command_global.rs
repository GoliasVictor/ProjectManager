use crate::commands::*;
use clap::{command, Args, Parser};
use clap::{CommandFactory, Subcommand};
use std::{env, path::PathBuf};

#[derive(Subcommand)]

pub enum Commands {
    Open(CommandOpen),
    Dir(CommandDir),
    List(CommandList),
    Run(CommandRun),
    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

#[derive(Args, Debug)]
pub struct GlobalArgs {
    #[arg(short, long, global = true)]
    pub root: Option<PathBuf>,
}

#[derive(Parser)]
#[command(name = "pm", id="pm")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub global: GlobalArgs,
}

impl Cli {
    pub fn execute(self) {
        let root = self
            .global
            .root
            .or(env::var("DEV_DIR").map(|p| PathBuf::from(&p)).ok())
            .expect("root and develoonebt dir is undefined");
        match self.command {
            Commands::List(command) => {
                command.execute(&root);
            }
            Commands::Run(command) => {
                command.execute(&root);
            }
            Commands::Dir(command) => command.execute(&root),
            Commands::Open(command) => command.execute(&root),
            Commands::Completions { shell } => {
                shell.generate(&mut Cli::command(), &mut std::io::stdout());
            }
        }
    }
}
