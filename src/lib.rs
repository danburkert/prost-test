extern crate bytes;
extern crate prost;
extern crate prost_types;

pub mod google {
    pub mod datastore {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/google.datastore.v1.rs"));
        }
    }
    pub mod r#type {
        include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));
    }
}
