use exitfailure::ExitFailure;
use failure::ResultExt;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cds")]
struct CdsOpt {
    /// Extracts chaincode
    #[structopt(short = "x", long)]
    extract_code: bool,

    /// .cds file to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let opt = CdsOpt::from_args();

    let content = std::fs::read_to_string(&opt.file)
        .with_context(|_| format!("could not read file `{}`", &opt.file.display()))?;
    println!("file content: {}", content);

    Ok(())
}
