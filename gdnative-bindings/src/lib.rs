#![allow(non_snake_case)] // because of the generated bindings.
#![allow(unused_imports)]
#![allow(clippy::unused_unit)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::identity_op)]
#![allow(clippy::unreadable_literal)]

pub use gdnative_core::*;

use crate::get_api;
use crate::sys;

use libc;
use std::ops::*;
use std::sync::Once;

include!(concat!(env!("OUT_DIR"), "/bindings_types.rs"));
include!(concat!(env!("OUT_DIR"), "/bindings_traits.rs"));
include!(concat!(env!("OUT_DIR"), "/bindings_methods.rs"));
