#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Ticker};
use panic_probe as _;

fn clock_config() -> embassy_stm32::Config {
    use embassy_stm32::rcc::*;

    let mut config = embassy_stm32::Config::default();

    // Configure to use the high speed internal oscillator (HSI).
    config.rcc.hsi = true;

    config
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize embassy
    let peripherals = embassy_stm32::init(clock_config());

    // Create a new output pin - PA9 is the green led on the Discovery board
    let mut led = Output::new(peripherals.PA9, Level::High, Speed::VeryHigh);

    // Create a new Ticker for the delay
    let mut ticker = Ticker::every(Duration::from_millis(500));

    loop {
        // Wait for the ticker to expire
        ticker.next().await;

        // Toggle the led
        led.toggle();
    }
}
