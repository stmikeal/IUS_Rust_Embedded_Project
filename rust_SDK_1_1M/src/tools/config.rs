use cortex_m::peripheral::SYST;
use stm32f1xx_hal::gpio::gpiob::{Parts, PB12};
use stm32f1xx_hal::gpio::{Output, PushPull};
use embedded_hal::prelude::_embedded_hal_timer_CountDown;
use stm32f1xx_hal::timer::CountDownTimer;



pub struct SDKConfig {
    pub timer: CountDownTimer<SYST>,
    pub led: PB12<Output<PushPull>>,
}