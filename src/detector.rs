use stm32f1xx_hal::gpio::{Input, Floating, gpiob::{PB5, PB6, PB7}};
use embedded_hal::digital::v2::InputPin;
pub enum DetectState {
    LeftMid,
    Left,
    Mid,
    Right,
    RightMid,
    LeftMidRight,
    Invalid,
}
pub struct Detector {
    do1: PB5<Input<Floating>>,
    do2: PB6<Input<Floating>>,
    do3: PB7<Input<Floating>>,
}

use super::Update;
impl Update for Detector {
    type Output = DetectState;
    fn update(&mut self) -> Self::Output {
        match (self.do1.is_high().unwrap(), self.do2.is_high().unwrap(), self.do3.is_high().unwrap()) {
            (true, true, true) => Self::Output::LeftMidRight,
            (true, true, false) => Self::Output::RightMid,
            (false, true, true) => Self::Output::LeftMid,
            (false, true, false) => Self::Output::Mid,
            (true, false, false) => Self::Output::Right,
            (false, false, true) => Self::Output::Left,
            (_, _, _) => Self::Output::Invalid,
        }
    }
}

pub struct DetectorBuilder {
    do1: Option<PB5<Input<Floating>>>,
    do2: Option<PB6<Input<Floating>>>,
    do3: Option<PB7<Input<Floating>>>,
}

impl DetectorBuilder {
    pub fn builder() -> Self {
        Self {
            do1: None,
            do2: None,
            do3: None,
        }
    }

    pub fn do1(mut self, pb05: PB5<Input<Floating>>) -> Self {
        self.do1 = Some(pb05);
        return self;
    }

    pub fn do2(mut self, pb06: PB6<Input<Floating>>) -> Self {
        self.do2 = Some(pb06);
        return self;
    }

    pub fn do3(mut self, pb07: PB7<Input<Floating>>) -> Self {
        self.do3 = Some(pb07);
        return self;
    }

    pub fn build(self) -> Detector {
        Detector {
            do1: self.do1.unwrap(),
            do2: self.do2.unwrap(),
            do3: self.do3.unwrap(),
        }
    }
}



