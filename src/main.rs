mod lib;

#[macro_use]
extern crate clap;

use clap::{Arg, App};
use exitfailure::ExitFailure;
use lib::ChaincodeDeploymentSpecFile;
use std::path::PathBuf;
use std::ffi::OsStr;

const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() -> Result<(), ExitFailure> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("OUTPUT")
            .help("Output file (defaults to stdout)")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Input file to process")
            .required(true)
            .index(1))
        .arg(Arg::with_name("extract")
            .short("x")
            .long("extract")
            .help("Extracts chaincode package tgz file")
            .conflicts_with_all(&["create", "name", "version", "language"]))
        .arg(Arg::with_name("create")
            .short("c")
            .long("create")
            .help("Creates a chaincode package cds file")
            .requires_all(&["name", "version", "language"]))
        .arg(Arg::with_name("name")
            .short("n")
            .long("name")
            .help("Name of the chaincode")
            .requires("create")
            .value_name("NAME"))
        .arg(Arg::with_name("version")
            .short("v")
            .long("version")
            .help("Version of the chaincode")
            .requires("create")
            .value_name("VERSION"))
        .arg(Arg::with_name("language")
            .short("l")
            .long("lang")
            .help("Language the chaincode is written in")
            .requires("create")
            .value_name("LANGUAGE")
            .possible_values(&["golang", "java", "node"]))
        .get_matches();

    let input_path = PathBuf::from(matches.value_of("INPUT").unwrap());
    let output_path = if let Some(s) = matches.value_of_os("output") {
        Some(PathBuf::from(s))
    } else {
        None
    };

    if matches.is_present("create") {
        let name = matches.value_of("name").unwrap().to_string();
        let version = matches.value_of("version").unwrap().to_string();
        let language = matches.value_of("language").unwrap().to_string();
        let path = format!("/Users/{}{}/{}", NAME.unwrap_or("cds"), VERSION.unwrap_or(""), input_path.file_stem().unwrap_or(OsStr::new("unknown")).to_string_lossy());
        let cds = ChaincodeDeploymentSpecFile::new(name, version, language, path, &input_path)?;

        let buffer = cds.encode()?;
        lib::write_output(&buffer, output_path)?;
    } else {
        let cds = ChaincodeDeploymentSpecFile::from(&input_path)?;
    
        if matches.is_present("extract") {
            let ccpkg = cds.ccpkg();
            lib::write_output(ccpkg, output_path)?;
        } else {
            let info = cds.format_info();
            let buffer = info.as_bytes();
            lib::write_output(&buffer, output_path)?;
        }
    }

    Ok(())
}
