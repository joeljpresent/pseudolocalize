//! Pseudolocalize is a pseudolocalization crate for Rust.
//! 
//! Pseudolocalization is a software testing method used for testing
//! internationalization aspects of software
//! (cf. [Wikipedia](https://en.wikipedia.org/wiki/Pseudolocalization)).
//! 
//! For now, this crate lets you transform a string to replace its ASCII 
//! letters by similar letter-like characters, usually letters with
//! diacritics.
//! 
//! ## Example
//! 
//! ``` rust
//! use pseudolocalize::Pseudolocalizer;
//! fn main() {
//!     // Basic example
//!     let pl = Pseudolocalizer::new();
//!     let s = pl.transform("The quick brown fox jumps over the lazy dog");
//!     assert_eq!(s, "[!!! Ŧℏë ʠûíçķ ƃŕøẅñ ƒøẍ ĵûɱƥŝ øṽëŕ țℏë łάẓƴ ďøǧ !!!]");
//! 
//!     // More complex example
//!     let pl = Pseudolocalizer::new()
//!                 .with_prefix("« ")
//!                 .with_suffix(" »")
//!                 .with_increase_percentage(30)
//!                 .with_extension_string(" Lôřè₥ ïƥƨú₥ôáñ δôℓôř ƨïƭ á₥èƭ");
//!     let s = pl.transform("The quick brown fox jumps over the lazy dog.");
//!     assert_eq!(s, "« Ŧℏë ʠûíçķ ƃŕøẅñ ƒøẍ ĵûɱƥŝ øṽëŕ țℏë łάẓƴ ďøǧ. Lôřè₥ ïƥƨú₥ô »");
//! }
//! ```

#![allow(dead_code)]

mod percent;
mod pseudolocalizer;
mod transform;
pub use pseudolocalizer::Pseudolocalizer;
pub use transform::{diacriticize, transform_str};
