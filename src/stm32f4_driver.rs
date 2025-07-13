use stm32f4xx_hal::{
    pac,
    prelude::*,
    gpio::{gpioa::PA5, Output, PushPull},
    rcc::Clocks,
};

/// Struct holding all initialized peripherals you want to use
pub struct DeviceResources {
    pub led: PA5<Output<PushPull>>,
    pub clocks: Clocks,
    // Add other fields as needed
}

/// Initialize peripherals and return them bundled
pub fn init(dp: pac::Peripherals) -> DeviceResources {
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze(); // Adjust as needed

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    let _ = led.set_low(); // Ensure LED is off

    DeviceResources { led, clocks }
}