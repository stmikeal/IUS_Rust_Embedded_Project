#![deny(unsafe_code)]
#![no_std]
#![no_main]

pub mod user;
pub mod tools;

use panic_halt as _;

use nb::block;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    timer::Timer,
};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use crate::user::main::Program;
use crate::user::SDKProgram::SDKProgram;

// Определяем входную функцию.
#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut config = tools::config::SDKConfig {
        timer: Timer::syst(cp.SYST, &clocks).start_count_down(10.hz()),
        led: gpiob.pb12.into_push_pull_output(&mut gpiob.crh),
    };
    let mut program = Program{
        config,
    };
    loop {
        program.main();
    }

}