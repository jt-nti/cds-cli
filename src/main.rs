mod cli;
mod lib;

use exitfailure::ExitFailure;
use lib::ChaincodeDeploymentSpecFile;

fn main() -> Result<(), ExitFailure> {
    let opt = cli::parse_args();

    let cds = ChaincodeDeploymentSpecFile::new(&opt.file)?;

    if opt.extract_code {
        cds.write_ccpkg()?;
    } else {
        cds.write_info();
    }

    Ok(())
}
