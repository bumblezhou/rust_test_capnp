//build.rs
fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("./schema/hello_world.capnp")
        .run().expect("schema compiler command");
}