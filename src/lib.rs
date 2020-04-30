pub mod fabric_protos {
    include!(concat!(env!("OUT_DIR"), "/fabric.rs"));
}

use prost::Message;
use std::io::Cursor;
use std::fs::File;
use std::io::Read;
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

    pub fn ccpkg(&self) -> &Vec<u8> {
        &self.cds.code_package
    }

    pub fn format_info(&self) -> String {
        let ccspec = &self.cds.chaincode_spec.as_ref().unwrap();
        let cctype = ccspec.r#type;
        let ccid = ccspec.chaincode_id.as_ref().unwrap();
        let ccpath = &ccid.path;
        let ccname = &ccid.name;
        let ccversion = &ccid.version;

        format!("Type: {}\nPath: {}\nName: {}\nVersion: {}\n", cctype, ccpath, ccname, ccversion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_cds() -> ChaincodeDeploymentSpecFile {
        let chaincode_type = fabric_protos::chaincode_spec::Type::Java;

        let chaincode_id = fabric_protos::ChaincodeId {
            path: "/tmp/fabtest".to_owned(),
            name: "fabtest".to_owned(),
            version: "1.0.0".to_owned()
        };
        
        let chaincode_spec = fabric_protos::ChaincodeSpec {
            r#type: chaincode_type as i32,
            chaincode_id: Some(chaincode_id),
            input: None,
            timeout: 42
        };

        let code_package = vec![42];

        let cds = fabric_protos::ChaincodeDeploymentSpec {
            chaincode_spec: Some(chaincode_spec),
            code_package: code_package
        };

        ChaincodeDeploymentSpecFile {
            cds: cds
        }
    }

    #[test]
    fn it_should_get_ccpkg() {
        let cds = mock_cds();
        let expected_pkg = vec![42];

        let ccpkg = cds.ccpkg();

        assert_eq!(ccpkg, &expected_pkg);
    }

    #[test]
    fn it_should_format_info() {
        let cds = mock_cds();

        let info = cds.format_info();

        assert_eq!(info, "Type: 4\nPath: /tmp/fabtest\nName: fabtest\nVersion: 1.0.0\n");
    }
}
