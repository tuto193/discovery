#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

fn get_next_led(x0: u8, y0: u8) -> [[u8; 5]; 5] {
    let mut led_matrix: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let x1, y1 = match (x0, y0) {
        (4, 0) => (4, 0),
        ()
        if y0 == 0 {
        if x0 == 4 {
            (4, 0)
        } else {
            (x0 + 1, 0)
        }
    } else if y0 == 4 {
        if x0 == 0 {
            (0, 4)
        } else {
            (x0 - 1, 4)
        }
    } else {
        x0
    };
    };
}

#[entry]
fn main() -> ! {
    // Necessary to get cmd output
    rtt_init_print!();

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // let mut row1 = board.display_pins.row1;
    let led_matrix = [
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
    ];
    loop {
        // timer.delay_ms(1000u16);
        // rprintln!("1000 ms passed");
        display.show(&mut timer, led_matrix, 1000);
        display.clear();
        timer.delay_ms(1000_u32);
    }
}
