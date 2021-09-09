#![allow(non_upper_case_globals)]
#![allow(clippy::enum_variant_names)]
pub mod accessibility;
pub mod background;
pub mod borders;
pub mod effects;
pub mod filters;
pub mod flexbox;
pub mod interactivity;
pub mod layouts;
pub mod sizing;
pub mod spacing;
pub mod tables;
pub mod transforms;
pub mod transition;
pub mod typography;

pub use self::effects::*;
use crate::*;
use std::{
    collections::{BTreeSet, HashMap},
    fmt::{Debug, Display, Formatter},
};
use tailwind_ast::{parse_i_px_maybe, parse_integer};
use tailwind_error::Result;
