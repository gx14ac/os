/* Target Arch */
OUTPUT_ARCH(riscv)

ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    
    kernel_start = .;

    text_start = .;

    /* テキストセクション */
    .text : {
        /* .text セクションを先頭に置く*/
        *(.text.entry)
        *(.text .text.*)
    }

    rodata_start = .;

    /* roデータセクション(Read only) */
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    data_start = .;

    /* データセクション */
    .data : {
        *(.data .data.*)
    }

    bss_start = .;

    /* bssセクション */
    .bss : {
        *(.sbss .bss .bss.*)
    }

    . = ALIGN(4K);
    kernel_end = .;
}