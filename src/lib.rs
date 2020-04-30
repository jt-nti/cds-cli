pub mod fabric_protos {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}

use prost::Message;
use std::io::Cursor;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use fabric_protos::ChaincodeDeploymentSpec;

pub struct ChaincodeDeploymentSpecFile {
    cds: ChaincodeDeploymentSpec
}

impl ChaincodeDeploymentSpecFile {
    pub fn new(file: &PathBuf) -> Result<ChaincodeDeploymentSpecFile, std::io::Error> {
        let mut buffer = Vec::new();
        let mut file = File::open(file)?;
        file.read_to_end(&mut buffer)?;

        let cds = fabric_protos::ChaincodeDeploymentSpec::decode(&mut Cursor::new(buffer))?;

        Ok(ChaincodeDeploymentSpecFile {
            cds: cds
        })
    }

    pub fn write_ccpkg(&self) -> Result<(), std::io::Error> {
        let ccpkg = &self.cds.code_package;

        io::stdout().write_all(&ccpkg)?;

        Ok(())
    }

    pub fn write_info(&self) {
        let ccspec = &self.cds.chaincode_spec.as_ref().unwrap();
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
}
