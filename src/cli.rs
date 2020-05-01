use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cds")]
pub struct Options {
    /// Output file (defaults to stdout)
    #[structopt(short, long, parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// Extracts chaincode package
    #[structopt(short = "x", long)]
    pub extract_code: bool,

    /// .cds file to process
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: PathBuf,
}

pub fn parse_args() -> Options {
    Options::from_args()
}
