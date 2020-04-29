use prost::Message;
use std::io::Cursor;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub mod fabric_protos {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}

pub fn read_cds(file: &PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = Vec::new();
    let mut file = File::open(file)?;
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn decode_cds(buf: &[u8]) -> Result<fabric_protos::ChaincodeDeploymentSpec, prost::DecodeError> {
    fabric_protos::ChaincodeDeploymentSpec::decode(&mut Cursor::new(buf))
}

pub fn extract_code(cds: &fabric_protos::ChaincodeDeploymentSpec) -> Result<(), std::io::Error> {
    let ccpkg = &cds.code_package;

    io::stdout().write_all(&ccpkg)?;

    Ok(())
}

pub fn show_info(cds: &fabric_protos::ChaincodeDeploymentSpec) {
    let ccspec = cds.chaincode_spec.as_ref().unwrap();
    let cctype = ccspec.r#type;
    let ccid = ccspec.chaincode_id.as_ref().unwrap();
    let ccpath = &ccid.path;
    let ccname = &ccid.name;
    let ccversion = &ccid.version;

    println!("Type: {}", cctype);
    println!("Path: {}", ccpath);
    println!("Name: {}", ccname);
    println!("Version: {}", ccversion);
}
