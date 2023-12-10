# ESP32-C3 Rust GPIO LED And Button Demo

## 开发板

9.9 元的合宙 ESP32-C3 开发板

开发板说明 https://wiki.luatos.com/chips/esp32c3/board.html

PCB 线路图 https://wiki.luatos.com/_static/bom/esp32c3.html

开发板支持两个 LED 和一个按钮

* LED D4 使用 GPIO12
* LED D5 使用 GPIO13
* BOOT 按钮使用 GPIO9（默认状态上拉 3.3V，按下接地 0V。上电时按住会进入 Download Boot 模式。上电时不按住，等正常启动后可以按下，可以从 GPIO9 读取高低电平）

## Build

```bash
cargo build --release --target riscv32imc-unknown-none-elf
```

你将会得到一个大小为 27kB 的二进制文件。

只是简单的功能不需要安装额外的 nightly toolchain。

## Flash

首先安装 espflash

```bash
cargo install espflash
```

然后通过 USB 线将 EPS32-C3 开发板连接到电脑。然后直行 flash 指令

```bash
espflash flash target/riscv32imc-unknown-none-elf/release/led
```

你还可以添加 `--monitor` 参数在写入 Flash 之后，直接读取程序运行时的串口输出。

```bash
espflash flash target/riscv32imc-unknown-none-elf/release/led --monitor
```
