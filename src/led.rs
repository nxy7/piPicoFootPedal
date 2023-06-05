use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    peripherals::{PIN_20, PIN_25},
};

#[embassy_executor::task]
pub async fn led_task(p20: PIN_20, p25: PIN_25) {
    let mut led = Output::new(p25, Level::High);
    let mut btn = Input::new(p20, Pull::Up);
    loop {
        btn.wait_for_any_edge().await;
        if btn.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
