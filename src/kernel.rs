// kernel.rs
// http://blog.thiago.me/raspberry-pi-bare-metal-programming-with-rust/

#![feature(start, lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]

extern crate rusty_metal_raspberry_pi;

//use rusty_metal_raspberry_pi::base;
use rusty_metal_raspberry_pi::gpio;
use rusty_metal_raspberry_pi::systimer;

mod matrix;

#[no_mangle]
pub extern "C" fn main() {

    for _ in 0..4 {
        gpio::digital_write(gpio::ACT_LED, true);
        systimer::delay_micros(100000);

        gpio::digital_write(gpio::ACT_LED, false);
        systimer::delay_micros(100000);
    }



    let mut update = systimer::timer_lower_bits();
    let mut led_state = false;

    let mut matrix = matrix::Matrix::new(1000);
    matrix.init();

    matrix.set(0, 0, true);
    matrix.set(1, 1, true);
    matrix.set(2, 2, true);
    matrix.set(3, 3, true);
    matrix.set(4, 4, true);
    matrix.set(5, 3, true);
    matrix.set(6, 2, true);

    matrix.set(0, 4, true);
    matrix.set(1, 3, true);
    matrix.set(2, 2, true);
    matrix.set(3, 1, true);
    matrix.set(4, 0, true);
    matrix.set(5, 1, true);
    matrix.set(6, 2, true);

    loop {
        let now = systimer::timer_lower_bits();

        matrix.update(now);

        if now - update > 200000 {
            gpio::digital_write(gpio::ACT_LED, led_state);
            led_state = !led_state;
            update = now;
        }
    }
}
