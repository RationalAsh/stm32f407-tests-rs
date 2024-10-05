#![no_std]
#![no_main]

use cortex_m::asm;
use panic_probe as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {
        for _ in 0..1_000_000 {
            asm::nop();
        }
    }
}
