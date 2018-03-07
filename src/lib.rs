#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
mod x86_64_macos;
#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
pub use x86_64_macos::*;
