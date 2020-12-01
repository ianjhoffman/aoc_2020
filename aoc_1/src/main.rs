use std::path::PathBuf;
use util::res::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let contents = util::file::read_to_string(opt.file)?;
    println!("Contents: {}", contents);

    Ok(()) // TODO
}