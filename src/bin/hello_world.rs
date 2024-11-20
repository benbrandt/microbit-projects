//! Prints a scrolling "Hello, world!" on a micro:bit
#![no_std]
#![no_main]
#![allow(clippy::used_underscore_binding)]

use embassy_executor::Spawner;
use microbit_bsp::{display::Brightness, Microbit};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Microbit::default();

    let mut display = board.display;

    display.set_brightness(Brightness::MAX);
    display.scroll("Hello, World!").await;
}
