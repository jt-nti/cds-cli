mod cli;
mod lib;

use exitfailure::ExitFailure;
use lib::ChaincodeDeploymentSpecFile;
use std::io::{self, Write};

fn main() -> Result<(), ExitFailure> {
    let opt = cli::parse_args();

    let cds = ChaincodeDeploymentSpecFile::new(&opt.file)?;

    if opt.extract_code {
        let ccpkg = cds.ccpkg();
        io::stdout().write_all(ccpkg)?;
    } else {
        let info = cds.format_info();
        let buffer = info.as_bytes();
        io::stdout().write_all(buffer)?;
    }

    Ok(())
}
