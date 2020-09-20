use anyhow::Result;
use cargo_concat::concat_source;
use clap::{Clap, Subcommand};
use std::fs::write;

#[derive(Clap)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Concat(InnerArgs),
}

#[derive(Clap)]
struct InnerArgs {
    #[clap(long, short, default_value = "Cargo.toml")]
    cargo_toml: String,

    #[clap(long, short, about = "target name")]
    bin: Option<String>,

    #[clap(long, short, about = "output path", default_value = "out.rs")]
    output: String,
}

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();
    let args: Args = Args::parse();

    match args.subcommand {
        SubCommand::Concat(args) => {
            let file_content = concat_source(&args.cargo_toml, args.bin.as_ref())?;
            write(&args.output, file_content)?;
            Ok(())
        }
    }
}
