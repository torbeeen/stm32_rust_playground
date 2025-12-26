use embassy_stm32::Peri;
use embassy_stm32::exti::{AnyChannel, ExtiInput};
use embassy_stm32::gpio::{AnyPin, Level, Output, Pull, Speed};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, signal::Signal};
use embassy_time::{Delay, Instant};

use hcsr04_async::{Config, DistanceUnit, Hcsr04, Now, TemperatureUnit};

use defmt::{error, info};

/// Provides system clock implementation for the HCSR04 ultrasonic sensor driver
/// by wrapping std::time::Instant
struct Clock;

impl Now for Clock {
    /// Returns current time in microseconds since system start
    fn now_micros(&self) -> u64 {
        Instant::now().as_micros()
    }
}

/// Fixed ambient temperature for distance calculations
/// Slight inaccuracy acceptable as we care more about consistent readings
const ULTRASONIC_TEMPERATURE: f64 = 21.5;

#[embassy_executor::task]
pub async fn ultrasonic_task(
    trigger_pin: Peri<'static, AnyPin>,
    echo_pin: Peri<'static, AnyPin>,
    echo_pin_exti: Peri<'static, AnyChannel>,
    signal: &'static Signal<ThreadModeRawMutex, u16>,
) {
    // Configure ultrasonic sensor with metric units
    let hcsr04_config: Config = Config {
        distance_unit: DistanceUnit::Centimeters,
        temperature_unit: TemperatureUnit::Celsius,
    };

    // Initialize GPIO pins for the HCSR04 sensor
    let trigger = Output::new(trigger_pin, Level::High, Speed::Low);
    let echo = ExtiInput::new(echo_pin, echo_pin_exti, Pull::None);
    let mut sensor = Hcsr04::new(trigger, echo, hcsr04_config, Clock, Delay);

    loop {
        let distance_cm = match sensor.measure(ULTRASONIC_TEMPERATURE).await {
            Ok(distance_cm) => distance_cm,
            Err(e) => {
                error!("Error: {}", e);
                //400.0
                continue;
            }
        };

        info!("D: {}", distance_cm);
        signal.signal(distance_cm as u16);
    }
}
