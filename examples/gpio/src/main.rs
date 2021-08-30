#![no_std]
#![no_main]
//use atsamg_hal as hal;
use atsamg_hal::target_device as pac;
use atsamg_hal::prelude::*;

use cortex_m::{asm};
use cortex_m_rt::entry;

use cortex_m_semihosting::{hprintln};

use panic_halt as _;


#[entry]
fn main() -> ! {

    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.PIOA.split();
    let mut outpin_h = gpioa.pa0.into_push_pull_output();
    outpin_h.set_low();
    let mut outpin_l = gpioa.pa1.into_push_pull_output();
    outpin_l.set_high();

    let input_pullup = gpioa.pa2.into_pull_up_input();
    let input_pulldown = gpioa.pa3.into_pull_down_input();
    let input_float = gpioa.pa4.into_floating_input();

    loop {
        asm::nop();
        hprintln!("{}, {}, {}", input_pullup.is_high(), input_pulldown.is_high(), input_float.is_high()).unwrap();
    }   
}
