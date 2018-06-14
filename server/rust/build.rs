extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &["../../proto"],
        input: &["../../proto/helloworld.proto"],
        rust_protobuf: true,
        ..Default::default()
    }).unwrap();
}
