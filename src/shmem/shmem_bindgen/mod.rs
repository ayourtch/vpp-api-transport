#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables, unused_mut)]
#![allow(unused)]

use std::fmt::{Debug, Error, Formatter};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
