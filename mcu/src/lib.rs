#![no_std]

mod ov7670;

use defmt_rtt as _;
use embassy_rp::dma::ChannelInstance;
use panic_probe as _;

embassy_rp::bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<embassy_rp::peripherals::PIO0>;
    I2C1_IRQ => embassy_rp::i2c::InterruptHandler<embassy_rp::peripherals::I2C1>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<embassy_rp::peripherals::DMA_CH0>;
});

pub async fn executor(spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut pio0 = embassy_rp::pio::Pio::new(p.PIO0, Irqs);
    let pio_gp10 = pio0.common.make_pio_pin(p.PIN_10);
    let pio_gp12 = pio0.common.make_pio_pin(p.PIN_12);
    let pio_gp13 = pio0.common.make_pio_pin(p.PIN_13);
    let pio_gp14 = pio0.common.make_pio_pin(p.PIN_14);
    let pio_gp15 = pio0.common.make_pio_pin(p.PIN_15);
    let pio_gp16 = pio0.common.make_pio_pin(p.PIN_16);
    let pio_gp17 = pio0.common.make_pio_pin(p.PIN_17);
    let pio_gp18 = pio0.common.make_pio_pin(p.PIN_18);
    let pio_gp19 = pio0.common.make_pio_pin(p.PIN_19);
    let pio_gp21 = pio0.common.make_pio_pin(p.PIN_21);

    setup_pio(&mut pio0, &pio_gp10, &pio_gp12, &pio_gp13, &pio_gp14, &pio_gp15, &pio_gp16, &pio_gp17, &pio_gp18, &pio_gp19, &pio_gp21).await;

    let hsync_signal_trigger_dma = p.DMA_CH3;

    {
        let mut xclk_pwm_config = embassy_rp::pwm::Config::default();
        xclk_pwm_config.invert_b = false;
        xclk_pwm_config.phase_correct = false;
        xclk_pwm_config.enable = true;
        xclk_pwm_config.compare_b = 2;
        xclk_pwm_config.top = 5;

        embassy_rp::pwm::Pwm::new_output_b(p.PWM_SLICE5, p.PIN_11, xclk_pwm_config);
    }
}

async fn setup_pio<'a>(
    pio0: &mut embassy_rp::pio::Pio<'a, embassy_rp::peripherals::PIO0>,
    gp10: &embassy_rp::pio::Pin<'a, embassy_rp::peripherals::PIO0>,
    gp12: &embassy_rp::pio::Pin<'a, embassy_rp::peripherals::PIO0>,
    gp13: &embassy_rp::pio::Pin<'a, embassy_rp::peripherals::PIO0>,
    gp14: &embassy_rp::pio::Pin<'a, embassy_rp::peripherals::PIO0>,
    gp15: &embassy_rp::pio::Pin<'a, embassy_rp::peripherals::PIO0>,
    gp16: &embassy_rp::pio::Pin<'a, embassy_rp::peripherals::PIO0>,
    gp17: &embassy_rp::pio::Pin<'a, embassy_rp::peripherals::PIO0>,
    gp18: &embassy_rp::pio::Pin<'a, embassy_rp::peripherals::PIO0>,
    gp19: &embassy_rp::pio::Pin<'a, embassy_rp::peripherals::PIO0>,
    gp21: &embassy_rp::pio::Pin<'a, embassy_rp::peripherals::PIO0>
) {
    let pio0_flow0= embassy_rp::pio::program::pio_asm!(
        ".wrap_target",
        "wait 1 gpio 10",
        "wait 1 gpio 20",
        "in pins, 8",
        "wait 0 gpio 20",
        ".wrap"
    );
    let pio0_prog0 = pio0.common.load_program(&pio0_flow0.program);

    let pio0_flow1 = embassy_rp::pio::program::pio_asm!(
        ".wrap_target",
        "reset_down:",
        "set x, 6",
        "wait_down:",
        "jmp pin, reset_down",
        "jmp x--, wait_down",
        "in null, 8",
        "irq 0 rel [13]",
        "reset_up:",
        "set x, 6",
        "wait_up:",
        "jmp pin, up_count",
        "jmp reset_up",
        "up_count:",
        "jmp x--, wait_up",
        "up_done:",
        ".wrap"
    );
    let pio0_prog1 = pio0.common.load_program(&pio0_flow1.program);

    {
        let mut pio0_sm0_conf = embassy_rp::pio::Config::default();
        pio0_sm0_conf.shift_in = embassy_rp::pio::ShiftConfig {
            auto_fill: true,
            direction: embassy_rp::pio::ShiftDirection::Left,
            threshold: 8
        };
        pio0_sm0_conf.fifo_join = embassy_rp::pio::FifoJoin::RxOnly;
        pio0_sm0_conf.set_in_pins(&[gp12, gp13, gp14, gp15, gp16, gp17, gp18, gp19]);
        pio0_sm0_conf.use_program(&pio0_prog0, &[]);

        pio0.sm0.set_config(&pio0_sm0_conf);
    }

    {
        let mut pio0_sm1_conf = embassy_rp::pio::Config::default();
        pio0_sm1_conf.shift_in = embassy_rp::pio::ShiftConfig {
            auto_fill: true,
            direction: embassy_rp::pio::ShiftDirection::Left,
            threshold: 8
        };
        pio0_sm1_conf.fifo_join = embassy_rp::pio::FifoJoin::RxOnly;
        pio0_sm1_conf.set_jmp_pin(gp10);
        pio0_sm1_conf.use_program(&pio0_prog1, &[]);

        pio0.sm1.set_config(&pio0_sm1_conf);
    }

    {
        let mut pio0_sm2_conf = embassy_rp::pio::Config::default();
        pio0_sm2_conf.shift_in = embassy_rp::pio::ShiftConfig {
            auto_fill: true,
            direction: embassy_rp::pio::ShiftDirection::Left,
            threshold: 8
        };
        pio0_sm2_conf.fifo_join = embassy_rp::pio::FifoJoin::RxOnly;
        pio0_sm2_conf.set_jmp_pin(gp21);
        pio0_sm2_conf.use_program(&pio0_prog1, &[]);

        pio0.sm2.set_config(&pio0_sm2_conf);
    }
}
