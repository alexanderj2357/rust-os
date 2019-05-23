#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]

extern crate compiler_builtins;

pub mod lang_items;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 600) {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

pub struct Gpio {
    pin: usize,
}

impl Gpio {
    pub fn pin(pin: usize) -> Gpio {
        Gpio { pin }
    }

    pub fn fsel(&self, mode: u32) {
        unsafe {
            let gpio_fsel1_reg: *mut u32 = GPIO_FSEL1.offset(self.fsel_register() as isize);
            gpio_fsel1_reg.write_volatile(gpio_fsel1_reg.read_volatile() &
                !(0b111 << self.fsel_offset()) |
                (mode << self.fsel_offset()));
        }
    }

    pub fn set(&self) {
        unsafe {
            let gpio_set0_reg: *mut u32 = GPIO_SET0.offset(self.gpio_register() as isize);
            gpio_set0_reg.write_volatile(1 << self.gpio_offset());
        }
    }

    pub fn clear(&self) {
        unsafe {
            let gpio_clr0_reg: *mut u32 = GPIO_CLR0.offset(self.gpio_register() as isize);
            gpio_clr0_reg.write_volatile(1 << self.gpio_offset());
        }
    }

    pub fn gpio_register(&self) -> usize {
        self.pin / 32
    }

    pub fn gpio_offset(&self) -> usize {
        self.pin - (self.pin / 32) * 32
    }

    pub fn fsel_register(&self) -> usize {
        self.pin / 10
    }

    pub fn fsel_offset(&self) -> usize {
        (self.pin - (self.pin / 10) * 10) * 3
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let gpio16 = Gpio::pin(16);
    // STEP 1: Set GPIO Pin 16 as output.
    gpio16.fsel(1);
    // STEP 2: Continuously set and clear GPIO 16.
    loop {
        gpio16.set();
        spin_sleep_ms(10);
        gpio16.clear();
        spin_sleep_ms(100);
    }
}
