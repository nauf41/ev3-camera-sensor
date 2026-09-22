#![no_std]
#![no_main]

use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    mcu::executor(spawner).await;
}
