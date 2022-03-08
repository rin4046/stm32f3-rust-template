#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f3xx_hal::{delay::Delay, pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpioe = dp.GPIOB.split(&mut rcc.ahb);
    let mut led = gpioe
        .pb3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    loop {
        led.toggle().unwrap();
        delay.delay_ms(1_000_u16);
    }
}
