use stm32f1xx_hal::gpio::{Output, PushPull, gpiob::{PB10, PB11, PB12, PB13, Parts}};
use embedded_hal::digital::v2::OutputPin;

#[derive(Debug, Clone)]
pub enum State {
    Forward,
    Backward,
    TuringLeft,
    TuringRight,
    UTuringRight,
    UTuringLeft,
    Stop,
}

pub struct WheelController {
    left: LeftWheel,
    right: RightWheel,
    pub state: State,
}

use super::detector::DetectState;
impl WheelController {
    pub fn force_stop(&mut self) {
        self.left.stop();
        self.right.stop();
    }
    pub fn set_state(&mut self, state: DetectState) {
        match state {
            DetectState::LeftMid => {
                self.state = State::TuringLeft;
            },
            DetectState::RightMid => {
                self.state = State::TuringRight;
            }
            DetectState::LeftMidRight => {
                self.state = State::Stop;
            },
            DetectState::Invalid => {
            },
            DetectState::Mid => {
                self.state = State::Forward;
            },
            DetectState::Right => {
                self.state = State::UTuringRight;
            },
            DetectState::Left => {
                self.state = State::UTuringLeft;
            }
        }
    }
}

use super::Update;
impl Update for WheelController {
    type Output=();

    fn update(&mut self) -> Self::Output {
        match &self.state {
            State::Forward => {
                self.left.forward();
                self.right.forward();
            },
            State::Backward => {
                self.left.backward();
                self.right.backward();
            },
            State::UTuringLeft => {
                self.left.backward();
                self.right.forward();
            },
            State::UTuringRight => {
                self.left.forward();
                self.right.backward();
            },
            State::Stop => {
                self.left.stop();
                self.right.stop();
            },
            State::TuringLeft => {
                self.left.stop();
                self.right.forward();
            },
            State::TuringRight => {
                self.right.stop();
                self.left.forward();
            }
        }
    }
}

struct LeftWheel {
    forward: PB11<Output<PushPull>>,
    backward: PB10<Output<PushPull>>,
}

impl LeftWheel {
    fn forward(&mut self) {
        self.forward.set_high();
        self.backward.set_low();
    }

    fn backward(&mut self) {
        self.forward.set_low();
        self.backward.set_high();
    }

    fn stop(&mut self) {
        self.forward.set_low();
        self.backward.set_low();
    }
}

struct RightWheel {
    forward: PB12<Output<PushPull>>,
    backward: PB13<Output<PushPull>>,
}

impl RightWheel {
    fn forward(&mut self) {
        self.forward.set_high();
        self.backward.set_low();
    }

    fn backward(&mut self) {
        self.forward.set_low();
        self.backward.set_high();
    }

    fn stop(&mut self) {
        self.forward.set_low();
        self.backward.set_low();
    }
}

pub struct WheelControllerBuilder {
    leftWheel: Option<LeftWheel>,
    rightWheel: Option<RightWheel>,
}

impl WheelControllerBuilder {
    pub fn builder() -> Self {
        Self { 
            leftWheel: None, 
            rightWheel:None,
        }
    }

    pub fn left_wheel(mut self, pb10: PB10<Output<PushPull>>, pb11: PB11<Output<PushPull>>) -> Self {
        self.leftWheel = Some(LeftWheel {
            forward: pb11,
            backward: pb10,
        });

        return self;
    }

    pub fn right_wheel(mut self, pb12: PB12<Output<PushPull>>, pb13: PB13<Output<PushPull>>) -> Self {
        self.rightWheel = Some(RightWheel {
            forward: pb12,
            backward: pb13,
        });

        return self;
    }

    pub fn build (mut self) -> WheelController {
        WheelController {
            left: self.leftWheel.unwrap(),
            right: self.rightWheel.unwrap(),
            state: State::Stop,
        }
    }
}