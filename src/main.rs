#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, signal::Signal};
use panic_probe as _;

mod tasks;

use crate::tasks::blink::blink_task;
use crate::tasks::button::button_task;
use crate::tasks::led_display::led_display_task;
use crate::tasks::ultrasonic_ranger::ultrasonic_task;

static SIGNAL_LED: Signal<ThreadModeRawMutex, u16> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());
    info!("Nucleo-F446RE Initialized!");

    spawner.spawn(blink_task(peripherals.PA5.into())).unwrap();
    spawner
        .spawn(button_task(
            peripherals.PC13.into(),
            peripherals.EXTI13.into(),
        ))
        .unwrap();
    spawner
        .spawn(led_display_task(
            peripherals.PA0.into(),
            peripherals.PA1.into(),
            &SIGNAL_LED,
        ))
        .unwrap();
    spawner
        .spawn(ultrasonic_task(
            peripherals.PA4.into(),
            peripherals.PB0.into(),
            peripherals.EXTI0.into(),
            &SIGNAL_LED,
        ))
        .unwrap();
}
