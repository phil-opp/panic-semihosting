//! Report panic messages to the host stderr using semihosting
//!
//! This crate contains an implementation of `panic_fmt` that logs panic messages to the host stderr
//! using [`cortex-m-semihosting`]. Currently, this crate only supports the ARM Cortex-M
//! architecture.
//!
//! [`cortex-m-semihosting`]: https://crates.io/crates/cortex-m-semihosting
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! extern crate panic_semihosting;
//!
//! fn main() {
//!     panic!("FOO")
//! }
//! ```
//!
//! ``` text
//! (gdb) monitor arm semihosting enable
//! (gdb) continue
//! Program received signal SIGTRAP, Trace/breakpoint trap.
//! rust_begin_unwind (args=..., file=..., line=8, col=5)
//!     at $CRATE/src/lib.rs:69
//! 69          asm::bkpt();
//! ```
//!
//! ``` text
//! $ openocd -f (..)
//! (..)
//! panicked at 'FOO', src/main.rs:6:5
//! ```

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(lang_items)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_semihosting as sh;

use core::fmt::{self, Write};
use cortex_m::{asm, interrupt};

use sh::hio;

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(
    args: core::fmt::Arguments,
    file: &'static str,
    line: u32,
    col: u32,
) -> ! {
    interrupt::disable();

    if let Ok(mut hstdout) = hio::hstdout() {
        (|| -> Result<(), fmt::Error> {
            hstdout.write_str("panicked at '")?;
            hstdout.write_fmt(args)?;
            hstdout.write_str("', ")?;
            hstdout.write_str(file)?;
            writeln!(hstdout, ":{}:{}", line, col)
        })().ok();
    }

    // OK to fire a breakpoint here because we know the microcontroller is connected to a debugger
    asm::bkpt();

    loop {}
}
