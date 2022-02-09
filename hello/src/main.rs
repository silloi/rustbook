// #![feature(alloc_error_handler)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;
extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};

use embedded_hal::serial::Write;
use stm32f4xx_hal::serial::{config::Config, config::StopBits, Serial};

use stm32f4xx_hal::i2c::I2c;

// use alloc::alloc::{GlobalAlloc, Layout};
// use alloc::vec::Vec;

// struct MyAllocator;

// unsafe impl GlobalAlloc for MyAllocator {
//     unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
//         0x2000_0100 as *mut u8
//     }
//     unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
// }

// #[global_allocator]
// static A: MyAllocator = MyAllocator;

// #[alloc_error_handler]
// fn on_oom(_layout: Layout) -> ! {
//     loop {}
// }

// use heapless::Vec;

#[entry]
fn main() -> ! {
    panic!("panic!!!");

    // let mut x = Vec::<_, 2>::new();
    // let _ = x.push(123);
    // let _ = x.push(456);
    // let _ = x.push(789);

    // let _ = hprintln!("{:?}", x);

    // let mut x = Vec::new();
    // x.push(123);
    // x.push(456);

    // let _ = hprintln!("{:?}", x);

    if let (Some(dp), Some(cp)) = (stm32::Peripherals::take(), stm32::CorePeripherals::take()) {
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let gpiod = dp.GPIOD.split();
        let mut led = gpiod.pd15.into_push_pull_output();

        let mut delay = Delay::new(cp.SYST, clocks);

        for _ in 0..5 {
            led.set_high().unwrap();
            delay.delay_ms(100_u32);

            led.set_low().unwrap();
            delay.delay_ms(100_u32);
        }

        let gpioc = dp.GPIOC.split();
        let config = Config::default()
            .baudrate(115200.bps())
            .wordlength_8()
            .parity_none()
            .stopbits(StopBits::STOP1);

        let pins = (
            gpioc.pc10.into_alternate_af8(),
            gpioc.pc11.into_alternate_af8(),
        );
        let mut uart4 = Serial::uart4(dp.UART4, pins, config, clocks).unwrap();

        let _ = uart4.write(b'H');
        let _ = uart4.write(b'e');
        let _ = uart4.write(b'l');
        let _ = uart4.write(b'l');
        let _ = uart4.write(b'o');
        let _ = uart4.flush();

        let gpiob = dp.GPIOB.split();

        let pins = (
            gpiob.pb8.into_alternate_af4().set_open_drain(),
            gpiob.pb9.into_alternate_af4().set_open_drain(),
        );
        let mut i2c = I2c::new(dp.I2C1, pins, 400.khz(), clocks);

        let _ = i2c.write(0, &[0]);
    }

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
