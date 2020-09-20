use anyhow::Result;
use cargo_concat::concat_source;
use clap::Clap;
use std::fs::write;

#[derive(Clap)]
#[clap(version = "0.1", author = "kenkoooo <kenkou.n@gmail.com>")]
struct Args {
    #[clap(long, short, default_value = "Cargo.toml")]
    cargo_toml: String,

    #[clap(long, short, about = "target name")]
    bin: Option<String>,

    #[clap(long, short, about = "output path", default_value = "out.rs")]
    output: String,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    let file_content = concat_source(&args.cargo_toml, args.bin.as_ref())?;
    write(&args.output, file_content)?;
    Ok(())
}
