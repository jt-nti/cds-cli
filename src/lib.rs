use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::ffi::OsStr;
use fabric_gateway_protos::ChaincodeDeploymentSpec;
use fabric_gateway_protos::ChaincodeSpec_Type;
use protobuf::Message;

const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub struct ChaincodeDeploymentSpecFile {
    cds: ChaincodeDeploymentSpec
}

impl ChaincodeDeploymentSpecFile {
    pub fn new(name: String, version: String, language: String, path: String, file: &PathBuf) -> Result<ChaincodeDeploymentSpecFile, std::io::Error> {
        let mut buffer = Vec::new();
        let mut file = File::open(file)?;
        file.read_to_end(&mut buffer)?;

        let mut ccid = fabric_gateway_protos::ChaincodeID::new();
        ccid.set_path(path);
        ccid.set_name(name);
        ccid.set_version(version);
        // let ccid = fabric_protos::ChaincodeId{
        //     path: path,
        //     name: name,
        //     version: version,
        // };

        let r#type = match language.as_str() {
            "golang" => ChaincodeSpec_Type::GOLANG,
            "java" => ChaincodeSpec_Type::JAVA,
            "node" => ChaincodeSpec_Type::NODE,
            _ => ChaincodeSpec_Type::UNDEFINED,
        };

        let mut chaincode_spec = fabric_gateway_protos::ChaincodeSpec::new();
        chaincode_spec.set_chaincode_id(ccid);
        chaincode_spec.set_field_type(r#type);
        chaincode_spec.set_timeout(0);
        // let chaincode_spec = fabric_gateway_protos::ChaincodeSpec{
        //     chaincode_id: Some(ccid),
        //     field_type: r#type,
        //     input: SingularPtrField::none(),
        //     timeout: 0,
        // };

        let mut cds = fabric_gateway_protos::ChaincodeDeploymentSpec::new();
        cds.set_chaincode_spec(chaincode_spec);
        cds.set_code_package(buffer);
        // let cds = fabric_gateway_protos::ChaincodeDeploymentSpec{
        //     chaincode_spec: Some(chaincode_spec),
        //     code_package: buffer,
        // };

        Ok(ChaincodeDeploymentSpecFile {
            cds: cds
        })
    }

    pub fn from(file: &PathBuf) -> Result<ChaincodeDeploymentSpecFile, std::io::Error> {
        let mut buffer = Vec::new();
        let mut file = File::open(file)?;
        file.read_to_end(&mut buffer)?;

        let cds = fabric_gateway_protos::ChaincodeDeploymentSpec::parse_from_bytes(&buffer)?;
        // let cds = fabric_protos::ChaincodeDeploymentSpec::decode(&mut Cursor::new(buffer))?;

        Ok(ChaincodeDeploymentSpecFile {
            cds: cds
        })
    }

    pub fn encode(&self) -> Result<Vec<u8>, std::io::Error>  {
        let mut buffer = Vec::new();
        buffer = self.cds.write_to_bytes()?;
        // &self.cds.encode(&mut buffer)?;
        Ok(buffer)
    }

    pub fn ccpkg(&self) -> &Vec<u8> {
        &self.cds.code_package
    }

    pub fn format_info(&self) -> String {
        let ccspec = &self.cds.chaincode_spec.as_ref().unwrap();
        let cctype = ccspec.field_type;
        let ccid = ccspec.chaincode_id.as_ref().unwrap();
        let ccpath = &ccid.path;
        let ccname = &ccid.name;
        let ccversion = &ccid.version;
        
        let typename = match cctype {
            ChaincodeSpec_Type::UNDEFINED => "Undefined",
            ChaincodeSpec_Type::GOLANG => "Golang",
            ChaincodeSpec_Type::NODE => "Node",
            ChaincodeSpec_Type::CAR => "Car",
            ChaincodeSpec_Type::JAVA => "Java",
        };

        format!("Type: {}\nPath: {}\nName: {}\nVersion: {}\n", typename, ccpath, ccname, ccversion)
    }
}

pub fn write_output(buffer: &[u8], output: Option<PathBuf>) -> Result<(), std::io::Error> {
    match output {
        Some(path) => {
            let mut file = File::create(path)?;
            file.write_all(buffer)?;
        },
        None => io::stdout().write_all(buffer)?
    }

    Ok(())
}

pub fn get_cds_path(input_path: &PathBuf, module: &str) -> String {
    let cds_path = if module.trim().is_empty() {
        format!("/Users/{}{}/{}", NAME.unwrap_or("cds"), VERSION.unwrap_or(""), input_path.file_stem().unwrap_or(OsStr::new("unknown")).to_string_lossy())
    } else {
        module.to_string()
    };

    cds_path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_cds() -> ChaincodeDeploymentSpecFile {
        let chaincode_type = fabric_gateway_protos::ChaincodeSpec_Type::JAVA;

        let mut chaincode_id = fabric_gateway_protos::ChaincodeID::new();
        chaincode_id.set_path("/tmp/fabtest".to_owned());
        chaincode_id.set_name("fabtest".to_owned());
        chaincode_id.set_version("1.0.0".to_owned());
        // let chaincode_id = fabric_protos::ChaincodeId {
        //     path: "/tmp/fabtest".to_owned(),
        //     name: "fabtest".to_owned(),
        //     version: "1.0.0".to_owned()
        // };
        
        let mut chaincode_spec = fabric_gateway_protos::ChaincodeSpec::new();
        chaincode_spec.set_field_type(chaincode_type);
        chaincode_spec.set_chaincode_id(chaincode_id);
        chaincode_spec.set_timeout(42);
        // let chaincode_spec = fabric_protos::ChaincodeSpec {
        //     r#type: chaincode_type as i32,
        //     chaincode_id: Some(chaincode_id),
        //     input: None,
        //     timeout: 42
        // };

        let code_package = vec![42];

        let mut cds = fabric_gateway_protos::ChaincodeDeploymentSpec::new();
        cds.set_chaincode_spec(chaincode_spec);
        cds.set_code_package(code_package);
        // let cds = fabric_protos::ChaincodeDeploymentSpec {
        //     chaincode_spec: Some(chaincode_spec),
        //     code_package: code_package
        // };

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

        assert_eq!(info, "Type: Java\nPath: /tmp/fabtest\nName: fabtest\nVersion: 1.0.0\n");
    }

    #[test]
    fn it_should_encode_data() {
        let cds = mock_cds();
        let expected_buffer = vec![10, 36, 8, 4, 18, 30, 10, 12, 47, 116, 109, 112, 47, 102, 97, 98, 116, 101, 115, 116, 18, 7, 102, 97, 98, 116, 101, 115, 116, 26, 5, 49, 46, 48, 46, 48, 32, 42, 26, 1, 42];

        let buffer = cds.encode();

        assert_eq!(buffer.unwrap(), expected_buffer);
    }

    #[test]
    fn it_should_get_default_cds_path() {
        let input_path = PathBuf::from("/home/conga/fabcar.tgz");
        let module = "";
        let cds_path = get_cds_path(&input_path, module);

        assert_eq!(cds_path, "/Users/cds0.5.0/fabcar".to_string());
    }

    #[test]
    fn it_should_get_golang_cds_path() {
        let input_path = PathBuf::from("/home/conga/fabcar.tgz");
        let module = "github.com/hyperledger/fabric-samples/chaincode/fabcar/go";
        let cds_path = get_cds_path(&input_path, module);

        assert_eq!(cds_path, "github.com/hyperledger/fabric-samples/chaincode/fabcar/go".to_string());
    }
}
