#[derive(Clone)]
pub struct Rgb888 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

static COLOR: embassy_sync::signal::Signal<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, Rgb888> = embassy_sync::signal::Signal::new();

#[embassy_executor::task]
pub async fn status_led(mut stm: embassy_rp::pio::StateMachine<'static, embassy_rp::peripherals::PIO1, 0>) {
    loop { // update
        let col = COLOR.wait().await;

        let dat = ((col.red.reverse_bits() as u32) << 16) | ((col.green.reverse_bits() as u32) << 8) | (col.blue.reverse_bits() as u32);

        stm.tx().push(dat);

        embassy_time::Timer::after_millis(1).await;
    }
}
