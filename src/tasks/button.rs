use defmt::info;
use embassy_stm32::Peri;
use embassy_stm32::exti::{AnyChannel, ExtiInput};
use embassy_stm32::gpio::{AnyPin, Pull};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn button_task(
    button_pin: Peri<'static, AnyPin>,
    button_pin_exti: Peri<'static, AnyChannel>,
) {
    // Configure PC13 (Blue Button) as Input.
    // We use Pull::None because the Nucleo board has an external resistor.
    let mut button = ExtiInput::new(button_pin, button_pin_exti, Pull::None);

    loop {
        button.wait_for_low().await;
        info!("Button Pressed!");
        Timer::after_millis(250).await;
        button.wait_for_high().await;
    }
}
