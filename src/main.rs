#![no_std]
#![no_main]

extern crate panic_halt;
extern crate cortex_m_rt;

use cortex_m_rt::entry;

use microbit::hal::prelude::*;
use microbit::Peripherals;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpio = p.GPIO.split();
    let mut led = gpio.pin14.into_push_pull_output();
    let _ = gpio.pin6.into_push_pull_output();
    let _ = led.set_high();

    loop {}
}
