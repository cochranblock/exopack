// Unlicense — public domain — cochranblock.org
//! exopack — testing augmentation: screenshot, video, interfaces, API mocks, triple sims, demo, baked_demo.
//!
//! ## Quick links (canonical names)
//! - TRIPLE SIMS: [`triple_sims::run`]
//! - Visual regression: [`screenshot::visual_regression`]
//! - HTTP test harness: [`interface::bind_random`], [`interface::http_client`]
//! - Mock server: [`mock::start_server`]
//!
//! P13-compressed names (`f60`, `t66`, …) remain as aliases for kova-internal
//! callers but are hidden from generated docs.

#![forbid(unsafe_code)]
// P13 compressed identifiers (t60, f61, s80) trigger naming and unused warnings
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

#[cfg(feature = "interface")]
pub mod interface;

#[cfg(feature = "mock")]
pub mod mock;

#[cfg(feature = "video")]
pub mod video;

#[cfg(feature = "screenshot")]
pub mod screenshot;

#[cfg(feature = "triple_sims")]
pub mod triple_sims;

#[cfg(feature = "devtools")]
pub mod devtools;

#[cfg(feature = "demo")]
pub mod demo;

#[cfg(feature = "baked_demo")]
pub mod baked_demo;

#[cfg(feature = "standards_check")]
pub mod standards_check;

#[cfg(feature = "harvest")]
pub mod harvest;

pub mod guard;
