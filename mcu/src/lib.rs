#![no_std]

mod status_led;

pub async fn executor(spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());

    spawner.spawn(status_led::status_led(
        embassy_rp::gpio::Output::new(p.PIN_5, embassy_rp::gpio::Level::Low),
        embassy_rp::gpio::Output::new(p.PIN_3, embassy_rp::gpio::Level::Low),
        embassy_rp::gpio::Output::new(p.PIN_4, embassy_rp::gpio::Level::Low)
    ).unwrap());
}
