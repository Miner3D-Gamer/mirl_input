//! A crate for handling keyboard and mouse related actions.
//!
//! This crate is to be used regardless of OS/env
//!
//! Related:
//! - `mirl_system`: Handles interactions with the Os/Env (and hardware)
//! - `mirl_windowing`: Create and manage visual windows
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(slice_index_methods)]
#![allow(incomplete_features)] // Idc anymore
#![feature(specialization)]
#![feature(const_cmp)]
#![feature(core_intrinsics)]
#![feature(checked_type_aliases)]
#![feature(derive_const)]
#![feature(const_clone)]
#![feature(const_default)]
#![feature(const_ops)]

/// Keyboard related stuff
pub mod keyboard;

/// Mouse related stuff
pub mod mouse;

/// Functions/Structs/Enums/Traits almost anyone using the lib will use
pub mod prelude;

/// Create specific file formats from Buffer like .bmp or .cur
pub mod formats;
