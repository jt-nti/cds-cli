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

    let cds = ChaincodeDeploymentSpecFile::from(&path);
    assert_eq!(cds.is_err(), true);
    assert_eq!(cds.err().unwrap().kind(), ErrorKind::NotFound);
}

#[test]
fn it_should_fail_if_file_is_not_a_cds_file() {
    let mut path = test_dir();
    path.push("invalid.cds");

    let cds = ChaincodeDeploymentSpecFile::from(&path);
    assert_eq!(cds.is_err(), true);
    assert_eq!(cds.err().unwrap().kind(), ErrorKind::InvalidData);
}

#[test]
fn it_should_successfully_read_a_cds_file() {
    let mut tgz_path = test_dir();
    tgz_path.push("test.tgz");

    let expected_cds = ChaincodeDeploymentSpecFile::new("conga".to_owned(), "73".to_owned(), "golang".to_owned(), "/Users/cds/test".to_owned(), &tgz_path).unwrap();

    let mut cds_path = test_dir();
    cds_path.push("test.cds");

    let cds = ChaincodeDeploymentSpecFile::from(&cds_path).unwrap();
    
    assert_eq!(cds.format_info(), expected_cds.format_info());
    assert_eq!(cds.ccpkg(), expected_cds.ccpkg());
}
