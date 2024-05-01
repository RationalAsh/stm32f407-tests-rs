#![no_std]
#![no_main]

use embassy_executor::Spawner;
use panic_probe as _;

fn clock_config() -> embassy_stm32::Config {
    let mut config = embassy_stm32::Config::default();

    config
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize embassy
    embassy_stm32::init(clock_config());

    loop {}
}
