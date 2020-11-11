#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused_must_use)]
use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;

use stm32f1xx_hal::{pac, prelude::*, timer::Timer, gpio::gpiob::Parts};

mod wheel;
use wheel::{WheelController, WheelControllerBuilder};

mod detector;
use detector::{Detector, DetectorBuilder};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let gpiob = dp.GPIOB.split(&mut rcc.apb2);
    
    let (mut wheel, mut detector) = init(gpiob);

    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(100.hz());

    loop {
        block!(timer.wait()).unwrap();
        wheel.set_state(detector.update());
        wheel.update();
        block!(timer.wait()).unwrap();
        wheel.force_stop();
    }
}

fn init(mut gpiob: Parts) -> (WheelController, Detector) {
    (
        WheelControllerBuilder::builder()
            .left_wheel(
                gpiob.pb10.into_push_pull_output(&mut gpiob.crh), 
                gpiob.pb11.into_push_pull_output(&mut gpiob.crh))
            .right_wheel(
                gpiob.pb12.into_push_pull_output(&mut gpiob.crh), 
                gpiob.pb13.into_push_pull_output(&mut gpiob.crh))
            .build(),
        DetectorBuilder::builder()
            .do1(gpiob.pb5.into_floating_input(&mut gpiob.crl))
            .do2(gpiob.pb6.into_floating_input(&mut gpiob.crl))
            .do3(gpiob.pb7.into_floating_input(&mut gpiob.crl))
            .build(),
    )
}


trait Update {
    type Output;
    fn update(&mut self) -> Self::Output;
}