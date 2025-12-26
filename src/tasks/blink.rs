use defmt::info;
use embassy_stm32::Peri;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn blink_task(led_pin: Peri<'static, AnyPin>) {
    // Configure PA5 (Onboard LD2) as an output
    let mut led = Output::new(led_pin, Level::High, Speed::Low);

    loop {
        info!("Blink!");
        led.toggle();
        Timer::after_millis(1000).await;
    }
}
