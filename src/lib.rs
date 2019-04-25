extern crate bytes;
extern crate prost;
extern crate prost_types;

pub mod datastore_proto {
    include!(concat!(env!("OUT_DIR"), "/google.datastore.v1.rs"));
}
