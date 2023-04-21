#![no_std]
#![no_main]
#![warn(rust_2018_idioms)]
#![allow(clippy::missing_safety_doc)]

#[macro_use]
mod macros;

mod arch;
mod console;
mod log;
#[cfg(target_os = "none")]
mod none;
#[cfg(target_os = "uefi")]
mod uefi;

use core::fmt::{self, Write};

// Workaround for https://github.com/hermitcore/rusty-loader/issues/117
use rusty_loader as _;

#[doc(hidden)]
fn _print(args: fmt::Arguments<'_>) {
	unsafe {
		console::CONSOLE.write_fmt(args).unwrap();
	}
}
