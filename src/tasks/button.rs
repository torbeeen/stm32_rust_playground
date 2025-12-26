use defmt::info;
use embassy_stm32::Peri;
use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn button_task(button_pin: Peri<'static, AnyPin>) {
    // Configure PC13 (Blue Button) as Input.
    // We use Pull::None because the Nucleo board has an external resistor.
    let button = Input::new(button_pin, Pull::None);

    loop {
        if button.is_low() {
            info!("Button Pressed!");
            while button.is_low() {
                Timer::after_millis(100).await;
            }
        }
        Timer::after_millis(100).await;
    }
}
