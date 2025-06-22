# RustOS

## Setup
* `rustup override set nightly`
* `rustup target add riscv64gc-unknown-none-elf`

## GDB Commands
* `file target/riscv64gc-unknown-none-elf/debug/kernel`
* `set architecture riscv:rv64`
* `set disassemble-next-line on`
* `target remote :1234`
  * `maintenence packet qqemu.PhyMemMode`
  * `maintenance packet Qqemu.PhyMemMode:1`
* `x/{lines}i ${reg}`
* `x/{lines}xg ${reg}`
* `p/x ${reg}`
* `si`
* `b {symbol} || b *{address}`
* `c`
