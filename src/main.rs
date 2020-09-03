#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32, serial};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();

        let mut led1 = gpioa.pa5.into_open_drain_output();

        led1.set_high().unwrap();
        
        let tx = gpioa.pa2.into_alternate_af7();
        let rx = gpioa.pa3.into_alternate_af7();
        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
        let cfg = serial::config::Config::default().baudrate(115_200.bps());

        let mut usart2 = serial::Serial::usart2(dp.USART2, (tx, rx), cfg, clocks).unwrap();

        loop {
            if usart2.is_rxne() {
                led1.set_low().unwrap();
                let w = usart2.read().unwrap();
                usart2.write(w);
                led1.set_high().unwrap();
            }
        }
    }

    loop {}
}