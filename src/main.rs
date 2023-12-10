#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, Delay, peripherals::Peripherals, prelude::*, IO};
use esp_println::println;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led4 = io.pins.gpio12.into_push_pull_output();
    let mut led5 = io.pins.gpio13.into_push_pull_output();
    let boot = io.pins.gpio9.into_floating_input();

    led4.set_high().unwrap();
    led5.set_low().unwrap();

    println!("Hello world!");
    loop {
        if boot.is_input_high() {
            println!("Button up...");
            led4.toggle().unwrap();
            led5.toggle().unwrap();
        } else {
            println!("Button down...");
        }
        delay.delay_ms(1000u32);
    }
}
