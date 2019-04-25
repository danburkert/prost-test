extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["src/googleapis/google/datastore/v1/datastore.proto"],
        &["src/googleapis"],
    )
    .unwrap();
}
