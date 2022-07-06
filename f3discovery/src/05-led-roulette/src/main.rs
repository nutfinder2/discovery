#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 50u8;
    leds[0].on().ok();
    loop {
        for idx in 0..leds.len() {
            let next_idx = (idx + 1) % 8;

            leds[next_idx].on().ok();
            delay.delay_ms(half_period);

            leds[idx].off().ok();
            delay.delay_ms(half_period);
        }
    }
}
