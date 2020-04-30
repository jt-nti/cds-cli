extern crate cds;

use cds::ChaincodeDeploymentSpecFile;
use std::path::PathBuf;
use std::io::ErrorKind;

fn test_dir() -> PathBuf {
    let this_file = file!();

    let mut path = PathBuf::from(this_file);
    path.pop();

    path
}

#[test]
fn it_should_fail_if_file_does_not_exist() {
    let mut path = test_dir();
    path.push("wot_no.cds");

    let cds = ChaincodeDeploymentSpecFile::new(&path);
    assert_eq!(cds.is_err(), true);
    assert_eq!(cds.err().unwrap().kind(), ErrorKind::NotFound);
}

#[test]
fn it_should_fail_if_file_is_not_a_cds_file() {
    let mut path = test_dir();
    path.push("invalid.cds");

    let cds = ChaincodeDeploymentSpecFile::new(&path);
    assert_eq!(cds.is_err(), true);
    assert_eq!(cds.err().unwrap().kind(), ErrorKind::InvalidData);
}
