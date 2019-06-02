#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(pointer_methods)]
#![feature(compiler_builtins_lib)]
#![no_builtins]

extern crate pi;
extern crate stack_vec;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

use pi::timer::spin_sleep_ms;
use pi::gpio::Gpio;

#[no_mangle]
pub extern "C" fn kmain() {
    unsafe {
        // STEP 1: Set GPIO Pin 16 as output.
        let mut led = Gpio::new(16).into_output();
        // STEP 2: Continuously set and clear GPIO 16.
        loop {
            led.set();
            spin_sleep_ms(500);
            led.clear();
            spin_sleep_ms(500);
        }
    }
}
