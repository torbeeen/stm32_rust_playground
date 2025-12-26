#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;

mod tasks;

use crate::tasks::blink::blink_task;
use crate::tasks::button::button_task;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());
    info!("Nucleo-F446RE Initialized!");

    spawner.spawn(blink_task(peripherals.PA5.into())).unwrap();
    spawner
        .spawn(button_task(peripherals.PC13.into(), peripherals.EXTI13.into()))
        .unwrap();
}
