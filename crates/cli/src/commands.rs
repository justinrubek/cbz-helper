use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    Rename(Rename),
}

#[derive(clap::Args, Debug)]
pub(crate) struct Rename {
    #[clap(subcommand)]
    pub command: RenameCommands,

    #[arg(short, long)]
    pub input: PathBuf,
    #[arg(short, long)]
    pub output_dir: PathBuf,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum RenameCommands {
    Tzp,
}
