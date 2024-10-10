#![no_std]
#![no_main]

use cortex_m::asm;
use panic_probe as _;
use stm32f4xx_hal::{gpio::GpioExt, pac};

#[cortex_m_rt::entry]
fn main() -> ! {
    // let mut green_led = Output::new(peripherals.PA9, Level::High, Speed::VeryHigh);
    // let mut red_led = Output::new(peripherals.PD5, Level::High, Speed::VeryHigh);
    // let mut green_led2 = Output::new(peripherals.PD12, Level::High, Speed::VeryHigh);
    // let mut orange_led = Output::new(peripherals.PD13, Level::High, Speed::VeryHigh);
    // let mut red_led2 = Output::new(peripherals.PD14, Level::High, Speed::VeryHigh);
    // let mut blue_led = Output::new(peripherals.PD15, Level::High, Speed::VeryHigh);

    let peripherals = pac::Peripherals::take().unwrap();
    let gpiod = &peripherals.GPIOD.split();

    loop {
        for _ in 0..1_000_000 {
            asm::nop();
        }
    }
}
