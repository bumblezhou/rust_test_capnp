//build.rs
fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("./schema/HelloWorld.capnp")
        .run().expect("schema compiler command");
}