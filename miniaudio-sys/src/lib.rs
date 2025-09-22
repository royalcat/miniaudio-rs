#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

pub mod util;

// include!(env!("MINIAUDIO_SYS_BINDINGS_FILE"));
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
