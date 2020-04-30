// use std::env;

fn main() {
    // env::set_var("OUT_DIR", "protoc-out");
    prost_build::compile_protos(&["cds.proto"],
                                &["protos"]).unwrap();
}
