use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cds")]
pub struct Options {
    /// Extracts chaincode
    #[structopt(short = "x", long)]
    pub extract_code: bool,

    /// .cds file to process
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: PathBuf,
}

pub fn parse_args() -> Options {
    Options::from_args()
}
