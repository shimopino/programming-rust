use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    // leg_devices: [fd::FileDesc; 8],
}

pub struct SpiderSenses {
    robot: Rc<SpiderRobot>, // 共有可能で不変な、参照カウントのRcを指している
    eyes: [u8; 32],
    // motion: Accelerometer,
}
