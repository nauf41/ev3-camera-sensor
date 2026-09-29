#![no_std]

mod status_led;
mod ov7675;

embassy_rp::bind_interrupts!(struct Irqs {
    PIO1_IRQ_0 => embassy_rp::pio::InterruptHandler<embassy_rp::peripherals::PIO1>;
});

pub async fn executor(spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut pio1 = embassy_rp::pio::Pio::new(p.PIO1, Irqs);
    // === Status LED === //
    {
        let pio_led_controller = embassy_rp::pio::program::pio_asm!(
            "
            .side_set 1

            .wrap_target
            routine_color_led_start:
            ; expect clock division 31.25 (250ns per clk)
                out x, 1 side 0 [2]
                jmp !x routine_color_led_start side 1
                nop side 1 [1]
            .wrap
            "
        );
        let mut pio1_stm0_cfg = embassy_rp::pio::Config::default();
        let gp0_pio = pio1.common.make_pio_pin(p.PIN_0);
        pio1_stm0_cfg.use_program(&pio1.common.load_program(&pio_led_controller.program), &[&gp0_pio]);
        pio1_stm0_cfg.shift_out = embassy_rp::pio::ShiftConfig {
            auto_fill: true,
            direction: embassy_rp::pio::ShiftDirection::Left,
            threshold: 24,
        };
        pio1_stm0_cfg.fifo_join = embassy_rp::pio::FifoJoin::TxOnly;
        pio1_stm0_cfg.clock_divider = fixed::FixedU32::<fixed::types::extra::U8>::from_num(31.25);
        pio1.sm0.set_config(&pio1_stm0_cfg);
        pio1.sm0.set_enable(true);

        spawner.spawn(status_led::status_led(pio1.sm0).unwrap());
    }
}
