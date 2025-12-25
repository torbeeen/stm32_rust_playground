#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm::nop;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    info!("Hello stm32");
    let mut cnt = 0;
    loop {
        for _ in 0..900_000 {
            nop();
        }
        info!("Cnt: {}", cnt);
        cnt += 1;
    }
}
