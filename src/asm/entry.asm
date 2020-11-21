# RiscV Assembly Doc -> https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md
# linker script で プログラムのエントリポイントを `_start` にしたので、そのラベルを使用する
    .section .text.entry
    .global _start

# 予約済みのスタックスペースをspに書き込み、rust_mainにジャンプする
_start:
    la sp, boot_stack_top
    call rust_main

    # .bssに.stackセクションを追加し、16Kバイトを0で埋める
    .section .bss.stack
    .global boot_stack
boot_stack:
    # Boot Stack Size(16K)
    .space 4096 * 16
    .global boot_stack_top
boot_stack_top: