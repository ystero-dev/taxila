#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

pub(crate) mod r17 {
    // TODO: Include the output of `build.rs` here.
    include!(concat!(env!("OUT_DIR"), "/ngap.rs"));
}
