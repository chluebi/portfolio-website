fn main() {
    prost_build::compile_protos(&["../proto/portfolio.proto"], &["../proto"]).unwrap();
}