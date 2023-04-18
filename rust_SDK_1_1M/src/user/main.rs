use embedded_hal::digital::v2::OutputPin;
use nb::block;
use crate::tools::config::SDKConfig as Config;
use crate::user::SDKProgram::SDKProgram;
use embedded_hal::prelude::_embedded_hal_timer_CountDown;


pub struct Program {
    pub config: Config,
}
impl SDKProgram for Program {
    fn init(&mut self)
    fn main(&mut self){
        block!(self.config.timer.wait()).unwrap();
        self.config.led.set_high().unwrap();
        block!(self.config.timer.wait()).unwrap();
        self.config.led.set_low().unwrap();
    }
}