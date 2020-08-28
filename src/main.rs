// #![no_std]
// #![no_main]

// // pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// // use panic_abort as _; // requires nightly
// // use panic_itm as _; // logs messages over ITM; requires ITM support
// // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
// use cortex_m_rt::entry;
// use cortex_m_semihosting::{debug, hprintln};
// use stm32f4::stm32f446;

// #[entry]
// fn main() -> ! {
//     // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
//     hprintln!("Hello, world").unwrap();
//     // debug::exit(debug::EXIT_SUCCESS);
//     loop {
//         // your code goes here
//     }
// }

#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();

        let mut led1 = gpioa.pa5.into_open_drain_output();
        let mut led2 = gpioa.pa6.into_open_drain_output();
        let mut led3 = gpioa.pa7.into_open_drain_output();
        let mut led4 = gpiob.pb6.into_open_drain_output();

        let mut sw1 = gpioa.pa1.into_pull_down_input();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            // On for 1s, off for 1s.
            led1.set_high().unwrap();
            led2.set_high().unwrap();
            led3.set_high().unwrap();
            // led4.set_high().unwrap();

            delay.delay_ms(1000_u32);

            led1.set_low().unwrap();
            led2.set_low().unwrap();
            led3.set_low().unwrap();
            // led4.set_low().unwrap();

            delay.delay_ms(1000_u32);
            if sw1.is_low().unwrap() {
                led4.toggle().unwrap();
            }
        }
    }

    loop {}
}