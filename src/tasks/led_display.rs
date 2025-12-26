use embassy_stm32::Peri;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, signal::Signal};
use embassy_time::Delay;

use tm1637_embedded_hal::{Brightness, TM1637, TM1637Builder, tokens::Async};

#[embassy_executor::task]
pub async fn led_display_task(
    dio_pin: Peri<'static, AnyPin>,
    clk_pin: Peri<'static, AnyPin>,
    signal: &'static Signal<ThreadModeRawMutex, u16>,
) {
    let dio = Output::new(dio_pin, Level::Low, Speed::Low);
    let clk = Output::new(clk_pin, Level::Low, Speed::Low);

    let mut tm: TM1637<4, Async, Output<'_>, Output<'_>, Delay> =
        TM1637Builder::new(clk, dio, Delay)
            .brightness(Brightness::L7)
            .delay_us(100)
            .build_async::<4>();

    tm.init().await.ok();

    loop {
        let distance_cm = signal.wait().await;

        let position = if distance_cm > 999 {
            0
        } else if distance_cm > 99 {
            1
        } else if distance_cm > 9 {
            2
        } else {
            3
        };

        tm.clear().await.ok();
        tm.options()
            .u16_4(distance_cm)
            .position(position)
            .display()
            .await
            .ok();
    }
}
