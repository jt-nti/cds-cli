use exitfailure::ExitFailure;

mod cli;
mod lib;

fn main() -> Result<(), ExitFailure> {
    let opt = cli::parse_args();

    let buffer = lib::read_cds(&opt.file)?;

    let cds = lib::decode_cds(&buffer)?;

    if opt.extract_code {
        lib::extract_code(&cds)?;
    } else {
        lib::show_info(&cds);
    }

    Ok(())
}
