use exitfailure::ExitFailure;
use prost::Message;
use std::io::Cursor;
use std::io::Read;
use std::io::{self, Write};
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

pub mod fabric_protos {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}

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

pub fn decode_cds(buf: &[u8]) -> Result<fabric_protos::ChaincodeDeploymentSpec, prost::DecodeError> {
    fabric_protos::ChaincodeDeploymentSpec::decode(&mut Cursor::new(buf))
}

fn main() -> Result<(), ExitFailure> {
    let opt = CdsOpt::from_args();

    let mut buffer = Vec::new();
    let mut file = File::open(&opt.file)?;
    file.read_to_end(&mut buffer)?;

    let cds = decode_cds(&buffer)?;

    if opt.extract_code {
        let ccpkg = cds.code_package;

        io::stdout().write_all(&ccpkg)?;
    } else {
        let ccspec = cds.chaincode_spec.unwrap();
        let cctype = ccspec.r#type;
        let ccid = ccspec.chaincode_id.unwrap();
        let ccpath = ccid.path;
        let ccname = ccid.name;
        let ccversion = ccid.version;
    
        println!("Type: {}", cctype);
        println!("Path: {}", ccpath);
        println!("Name: {}", ccname);
        println!("Version: {}", ccversion);
    }

    Ok(())
}
