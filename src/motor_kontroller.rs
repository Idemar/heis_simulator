use libc::c_double;

use crate::bygninger::Bygning;
use crate::fysikk::HeisStat;

#[link(name = "motor1")]
unsafe extern "C" {
    pub fn motor1_juster_motor(target_styrke: c_double) -> c_double;
}

#[link(name = "motor2")]
unsafe extern "C" {
    pub fn motor2_juster_motor(target_styrke: c_double) -> c_double;
}

#[link(name = "motor3")]
unsafe extern "C" {
    pub fn motor3_juster_motor(target_styrke: c_double) -> c_double;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MotorInput {
    Motor1 { target_styrke: f64 },
    Motor2 { target_styrke: f64 },
    Motor3 { target_styrke: f64 },
}

pub trait MotorDriver {
    fn juster_motor(&self, input: MotorInput);
}

struct Motor1;
impl MotorDriver for Motor1 {
    fn juster_motor(&self, input: MotorInput) {
        if let MotorInput::Motor1 { target_styrke } = input {
            unsafe {
                motor1_juster_motor(target_styrke);
            }
        }
    }
}

struct Motor2;
impl MotorDriver for Motor2 {
    fn juster_motor(&self, input: MotorInput) {
        if let MotorInput::Motor2 { target_styrke } = input {
            unsafe {
                motor2_juster_motor(target_styrke);
            }
        }
    }
}

struct Motor3;
impl MotorDriver for Motor3 {
    fn juster_motor(&self, input: MotorInput) {
        if let MotorInput::Motor3 { target_styrke } = input {
            unsafe {
                motor3_juster_motor(target_styrke);
            }
        }
    }
}

pub trait MotorKontroller {
    fn juster_motor(&self, f: f64);
    fn max_styrke(&self) -> f64;
}

pub struct MotorKontroller1 {
    motor: Motor1,
}

pub fn ny_motor_kontroller1() -> Box<dyn MotorKontroller> {
    Box::new(MotorKontroller1 { motor: Motor1 })
}

impl MotorKontroller for MotorKontroller1 {
    fn juster_motor(&self, f: f64) {
        self.motor
            .juster_motor(MotorInput::Motor1 { target_styrke: f })
    }
    fn max_styrke(&self) -> f64 {
        50000.0
    }
}

pub struct MotorKontroller2 {
    motor: Motor2,
}

pub fn ny_motor_kontroller2() -> Box<dyn MotorKontroller> {
    Box::new(MotorKontroller2 { motor: Motor2 })
}

impl MotorKontroller for MotorKontroller2 {
    fn juster_motor(&self, f: f64) {
        self.motor
            .juster_motor(MotorInput::Motor2 { target_styrke: f })
    }
    fn max_styrke(&self) -> f64 {
        100000.0
    }
}

pub struct MotorKontroller3 {
    motor: Motor3,
}

pub fn ny_motor_kontroller3() -> Box<dyn MotorKontroller> {
    Box::new(MotorKontroller3 { motor: Motor3 })
}

impl MotorKontroller for MotorKontroller3 {
    fn juster_motor(&self, f: f64) {
        self.motor
            .juster_motor(MotorInput::Motor3 { target_styrke: f })
    }
    fn max_styrke(&self) -> f64 {
        90000.0
    }
}
