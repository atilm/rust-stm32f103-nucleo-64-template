//! Prints the user LED of the Nucleo Board
//! ---

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f1xx_hal::{delay::Delay, pac, prelude::*}; // STM32F1 specific functions
#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp =  cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let mut gpio_a = dp.GPIOA.split();

    let mut led = gpio_a.pa5.into_push_pull_output(&mut gpio_a.crl);

    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high();
        delay.delay_ms(1_00_u16);
        led.set_low();
        delay.delay_ms(1_00_u16);
    }
}

