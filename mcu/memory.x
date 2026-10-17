MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100

    RAM       : ORIGIN = 0x21000000, LENGTH = 64K
    BANK_1       : ORIGIN = 0x21010000, LENGTH = 64K
    BANK_2       : ORIGIN = 0x21020000, LENGTH = 64K
    BANK_3       : ORIGIN = 0x21030000, LENGTH = 64K
    SCRATCH_A : ORIGIN = 0x20040000, LENGTH = 4K
    SCRATCH_B : ORIGIN = 0x20041000, LENGTH = 4K
}

SECTIONS {
    .dma_linbuf_0 (NOLOAD) : ALIGN(4) {
        *(.dma_linbuf_0 .dma_linbuf_0.*);
        . = ALIGN(4);
    } > SCRATCH_A

    .dma_ord_0 (NOLOAD) : ALIGN(4) {
        *(.dma_ord_0 .dma_ord_0.*);
        . = ALIGN(256);
    } > SCRATCH_A

    .dma_dat_0 (NOLOAD) : ALIGN(4) {
        *(.dma_dat_0 .dma_dat_0.*);
        . = ALIGN(256);
    } > SCRATCH_A

    .dma_linbuf_1 (NOLOAD) : ALIGN(4) {
        *(.dma_linbuf_1 .dma_linbuf_1.*);
        . = ALIGN(4);
    } > SCRATCH_B

    .dma_ord_1 (NOLOAD) : ALIGN(4) {
        *(.dma_ord_1 .dma_ord_1.*);
        . = ALIGN(4);
    } > SCRATCH_B

    .dma_dat_1 (NOLOAD) : ALIGN(4) {
        *(.dma_dat_1 .dma_dat_1.*);
        . = ALIGN(4);
    } > SCRATCH_B

    .dummy (NOLOAD) : ALIGN(1) {
        *(.dummy .dummy.*);
        . = ALIGN(1);
    } > SCRATCH_B

    .buf1 (NOLOAD) : ALIGN(4) {
        *(.buf1 .buf1.*);
    } > BANK_1

    .buf2 (NOLOAD) : ALIGN(4) {
        *(.buf2 .buf2.*);
    } > BANK_2

    .buf3 (NOLOAD) : ALIGN(4) {
        *(.buf3 .buf3.*);
    } > BANK_3
} INSERT AFTER .bss;