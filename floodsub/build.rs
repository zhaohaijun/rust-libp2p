extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/protobuf_structs",
        input: &["src/rpc.proto"],
        includes: &["src"],
        customize: Default::default(),
    }).expect("protoc failed to run");
}
