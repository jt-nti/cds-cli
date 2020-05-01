mod cli;
mod lib;

use exitfailure::ExitFailure;
use lib::ChaincodeDeploymentSpecFile;

fn main() -> Result<(), ExitFailure> {
    let opt = cli::parse_args();

    let cds = ChaincodeDeploymentSpecFile::new(&opt.file)?;

    if opt.extract_code {
        let ccpkg = cds.ccpkg();
        lib::write_output(ccpkg, opt.output)?;
    } else {
        let info = cds.format_info();
        let buffer = info.as_bytes();
        lib::write_output(&buffer, opt.output)?;
    }

    Ok(())
}
