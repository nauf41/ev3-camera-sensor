use core::str::Lines;

#[repr(align(16))]
#[derive(Copy, Clone)]
struct DmaOrder {
    pub read_addr: u32,
    pub write_addr: u32,
    pub transfer_count: u32,
    pub control: u32,
}

const fn generate_8bit_lut() -> [u8; 256] {
    let mut lut = [0u8; 256];
    let digit_table = [7-6, 7-4, 7-2, 7-0, 7-1, 7-3, 7-5, 7-7];

    let mut idx = 0;
    while idx < 8 {
        let mut res_val = 0u8;

        let mut digit = 0;
        while digit < 8 {
            if (idx & (1 << digit)) != 0 {
                res_val |= 1 << digit_table[digit];
            }

            digit+=1;
        }

        lut[idx] = res_val;

        idx+=1;
    }

    lut
}

#[unsafe(link_section = ".data")]
#[used]
static table_raw_byte_to_u8: [u8; 256] = generate_8bit_lut();

#[unsafe(link_section = ".dma_ord_0")]
#[used]
static dma_orders_0: [DmaOrder; 160] = [DmaOrder { read_addr: 0, write_addr: 0, transfer_count: 0, control: 0 }; 160];

#[unsafe(link_section = ".dma_ord_1")]
static dma_orders_1: [DmaOrder; 160] = [DmaOrder { read_addr: 0, write_addr: 0, transfer_count: 0, control: 0 }; 160];

#[unsafe(link_section = ".dma_linbuf_0")]
#[used]
static _dma_line_buffer_0: [u8; 5*16*16] = [0u8; 5*16*16];

#[unsafe(link_section = ".dma_linbuf_1")]
#[used]
static _dma_line_buffer_1: [u8; 5*16*16] = [0u8; 5*16*16];

#[unsafe(link_section = ".dma_dat_0")]
#[used]
static dma_data_0: [u8; 16*16] = [0u8; 16*16];

#[unsafe(link_section = ".dma_dat_1")]
#[used]
static dma_data_1: [u8; 16*16-1] = [0u8; 16*16-1];

#[unsafe(link_section = ".dummy")]
#[used]
static dummy: u8 = 0u8;


#[unsafe(link_section = ".buf1")]
#[used]
static _buf1: [u8; 64*1024] = [0u8; 64*1024];

#[unsafe(link_section = ".buf2")]
#[used]
static _buf2: [u8; 64*1024] = [0u8; 64*1024];

#[unsafe(link_section = ".buf3")]
#[used]
static _buf3: [u8; 64*1024] = [0u8; 64*1024];

pub enum TransferState {
    OddLine,
    EvenLine,
}

pub enum TransferQuality {
    Vga,
    QuarterVga,
}

impl TransferQuality {
    fn get_larger_edge_length(&self) -> u32 {
        match self {
            TransferQuality::Vga => 640,
            TransferQuality::QuarterVga => 320,
        }
    }
    
    fn get_shorter_edge_length(&self) -> u32 {
        match self {
            TransferQuality::Vga => 480,
            TransferQuality::QuarterVga => 240,
        }
    }
}

fn setup_hsync_signal_trigger_dma(ch_id: u8, chain_to: u8, transfer_quality: TransferQuality) {
    let PIO0_SM1_RX_FIFO_ADDRESS: u32 = 0x5020_0020;

    let ch = embassy_rp::pac::DMA.ch(ch_id as usize);
    
    ch.read_addr().write_value(PIO0_SM1_RX_FIFO_ADDRESS);
    ch.write_addr().write_value(core::ptr::addr_of!(dummy) as u32);
    ch.trans_count().write_value(transfer_quality.get_larger_edge_length());
    ch.ctrl_trig().write(|w| {
        w.set_en(true);
        w.set_data_size(embassy_rp::pac::dma::vals::DataSize::SIZE_BYTE);
        w.set_incr_read(false);
        w.set_incr_write(false);
        w.set_chain_to(chain_to);
        w.set_treq_sel(embassy_rp::pac::dma::vals::TreqSel::PIO0_RX1);
    });
}

fn setup_vsync_signal_trigger_dma(ch_id: u8) {
    let ch = embassy_rp::pac::DMA.ch(ch_id as usize);
    
    ch.read_addr().write_value(0x5020_0028);
    ch.write_addr().write_value(0x2004_1FFF);
    ch.trans_count().write_value(u32::MAX);
    ch.ctrl_trig().write(|w| {
        w.set_en(true);
        w.set_data_size(embassy_rp::pac::dma::vals::DataSize::SIZE_BYTE);
        w.set_incr_read(false);
        w.set_incr_write(false);
        w.set_chain_to(ch_id);
        w.set_treq_sel(embassy_rp::pac::dma::vals::TreqSel::PIO0_RX2);
    });
}

fn setup_queue_to_dma_dma(ch_id: u8, transfer_state: TransferState) {
    let dma_queue_begin_addr = match transfer_state {
        TransferState::OddLine => 0x2004_0500,
        TransferState::EvenLine => 0x2004_1500,
    };
    let dma1_reg_addr = 0x5000_0040;

    let ch = embassy_rp::pac::DMA.ch(ch_id as usize);
    
    ch.read_addr().write_value(dma_queue_begin_addr);
    ch.write_addr().write_value(dma1_reg_addr);
    ch.trans_count().write_value(4);
    ch.ctrl_trig().write(|w| {})
}
