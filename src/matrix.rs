/// Matrix
///
/// Draw to a matrix of leds

use rusty_metal_raspberry_pi::gpio;
use rusty_metal_raspberry_pi::base;
use rusty_metal_raspberry_pi::systimer;

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 5;

const COL_PINS: [u32; WIDTH] = [23, 24, 25, 8, 7, 1, 12];
const ROW_PINS: [u32; HEIGHT] = [19, 26, 16, 20, 21];

pub struct Matrix {
    matrix: [[bool; HEIGHT + 1]; WIDTH + 1],
    last_update: u32,
    last_x: usize,
    delay: u32,
}

impl Matrix {
    pub fn new(delay: u32) -> Matrix {
        Matrix {
            matrix: [[false; HEIGHT + 1]; WIDTH + 1],
            last_update: 0,
            last_x: 6,
            delay: delay,
        }
    }

    pub fn init(&self) {
        for i in 0..WIDTH {
            gpio::set_mode(COL_PINS[i], gpio::Modes::Out);
            gpio::digital_write(COL_PINS[i], true);
            //systimer::delay_micros(100000);
        }

        for i in 0..HEIGHT {
            gpio::set_mode(ROW_PINS[i], gpio::Modes::Out);
            gpio::digital_write(ROW_PINS[i], true);
            systimer::delay_micros(50000);
        }


        for i in 0..WIDTH {
            gpio::digital_write(COL_PINS[i], false);
        }

        for i in 0..HEIGHT {
            gpio::digital_write(ROW_PINS[i], false);
        }
    }

    pub fn set(&mut self, x: usize, y: usize, val: bool) {
        if x < WIDTH && y < HEIGHT {
            self.matrix[x][y] = val;
        }
    }

    pub fn update(&mut self, now: u32) {
        if now - self.last_update > self.delay {

            base::nothing();

            // The new x to use
            let x = if self.last_x + 1 >= WIDTH {
                0
            } else {
                self.last_x + 1
            };

            // Activate the new rows
            gpio::digital_write(ROW_PINS[0], self.matrix[x][0]);
            gpio::digital_write(ROW_PINS[1], self.matrix[x][1]);
            gpio::digital_write(ROW_PINS[2], self.matrix[x][2]);
            gpio::digital_write(ROW_PINS[3], self.matrix[x][3]);
            gpio::digital_write(ROW_PINS[4], self.matrix[x][4]);

            // Turn on the current column to update
            gpio::digital_write(COL_PINS[x], true);

            // Turn off the last updated column
            gpio::digital_write(COL_PINS[self.last_x], false);


            self.last_x = x;

            self.last_update = now;
        }
    }
}
