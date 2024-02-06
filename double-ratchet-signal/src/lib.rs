//! Crate documentation provided in README.md

// TODO: include README.md documentation: https://github.com/rust-lang/rust/issues/44732
// FIXED(myl7): Currently no need for an extra `doc` folder
// TODO: test examples in README.md
// FIXED(myl7): A test is available in the `signal.rs`

#![no_std]
// Because it would report errors like too similar variable names, which is hard to fix without largely update the code
// #![warn(clippy::pedantic)]
// #![warn(missing_docs)]

extern crate alloc;
extern crate double_ratchet_signal_cbc_aes256_pkcs7_compact as cbc_aes256_pkcs7_compact;
#[cfg(feature = "std")]
extern crate std;

pub mod signal;
