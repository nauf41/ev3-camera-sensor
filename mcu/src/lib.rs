#![no_std]
mod ov7670;

embassy_rp::bind_interrupts!(struct Irqs {
});

pub async fn executor(spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());
}
