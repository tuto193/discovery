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
use rtt_target::{
    // rprintln,
    rtt_init_print
};

fn get_next_led(coords: (usize, usize)) -> (usize, usize) {
    match coords {
        (4, y) if y < 4 => (4, y + 1),
        (x, 0) if x < 4 => (x + 1, 0),
        (0, y) if y > 0 => (0, y - 1),
        (x, 4) if x > 0 => (x - 1, 4),
        (_, _) => (0, 0),
    }
}

#[entry]
fn main() -> ! {
    // Necessary to get cmd output
    rtt_init_print!();

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // let mut row1 = board.display_pins.row1;
    let mut led_matrix = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut coord : (usize, usize) = (0, 0);
    loop {
        // rprintln!("1000 ms passed");
        led_matrix[coord.0][coord.1] = 0;
        coord = get_next_led(coord);
        led_matrix[coord.0][coord.1] = 1;
        display.show(&mut timer, led_matrix, 50);
    }
}
