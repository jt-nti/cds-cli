mod lib;

#[macro_use]
extern crate clap;

use clap::{Arg, App};
use exitfailure::ExitFailure;
use lib::ChaincodeDeploymentSpecFile;
use std::path::PathBuf;

fn main() -> Result<(), ExitFailure> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("OUTPUT")
            .help("Sets an optional output file (defaults to stdout)")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("input file to process")
            .required(true)
            .index(1))
        .arg(Arg::with_name("extract")
            .short("x")
            .long("extract")
            .help("Extracts chaincode package tgz file"))
        .get_matches();

    let cds_path = PathBuf::from(matches.value_of("INPUT").unwrap());
    let cds = ChaincodeDeploymentSpecFile::new(&cds_path)?;

    let output_path = if let Some(s) = matches.value_of_os("output") {
        Some(PathBuf::from(s))
    } else {
        None
    };

    if matches.is_present("extract") {
        let ccpkg = cds.ccpkg();
        lib::write_output(ccpkg, output_path)?;
    } else {
        let info = cds.format_info();
        let buffer = info.as_bytes();
        lib::write_output(&buffer, output_path)?;
    }

    Ok(())
}
