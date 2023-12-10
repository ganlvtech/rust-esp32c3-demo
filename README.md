# ESP32-C3 Rust GPIO LED And Button Demo

## 开发板

9.9 元的合宙 ESP32-C3 开发板

开发板说明 https://wiki.luatos.com/chips/esp32c3/board.html

PCB 线路图 https://wiki.luatos.com/_static/bom/esp32c3.html

开发板支持两个 LED 和一个按钮

* LED D4 使用 GPIO12
* LED D5 使用 GPIO13
* BOOT 按钮使用 GPIO9（默认状态上拉 3.3V，按下接地 0V。上电时按住会进入 Download Boot 模式。上电时不按住，等正常启动后可以按下，可以从 GPIO9 读取高低电平）

## 编译

```bash
cargo build --release --target riscv32imc-unknown-none-elf
```

你将会得到一个大小为 27kB 的 ELF 文件。

只是简单的功能不需要安装额外的 nightly toolchain。

## 将程序写入 Flash

首先安装 espflash

```bash
cargo install espflash
```

然后通过 USB 线将 EPS32-C3 开发板连接到电脑。然后执行 flash 指令

```bash
espflash flash target/riscv32imc-unknown-none-elf/release/led
```

你还可以添加 `--monitor` 参数在写入 Flash 之后，直接读取程序运行时的串口输出。

```bash
espflash flash target/riscv32imc-unknown-none-elf/release/led --monitor
```

## 关于镜像

需要注意，flash 中的[镜像格式](https://docs.espressif.com/projects/esptool/en/latest/esp32c3/advanced-topics/firmware-image-format.html)并不是 ELF 格式，需要使用 espflash 进行转换。你可以预览将要写入 flash 的内容。

```bash
espflash save-image --merge --chip esp32c3 target/riscv32imc-unknown-none-elf/release/led led
```

* 文件的开头是 espflash 中内置的[二级引导程序](https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32c2/api-guides/startup.html#second-stage-bootloader) [bootloader.bin](https://github.com/esp-rs/espflash/blob/main/espflash/resources/bootloaders/esp32c3-bootloader.bin) 镜像（[源码](https://github.com/espressif/esp-idf/tree/master/components/bootloader)）
* 0x8000 处是[分区表](https://docs.espressif.com/projects/esp-idf/zh_CN/v5.1.2/esp32c3/api-guides/partition-tables.html)
* 0x10000 处是应用程序的镜像
* 空余位置均填充 0xff 字节

如果不加 `--merge` 则只包含应用程序的镜像。
