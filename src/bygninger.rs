use heis_driver::{HeisDriver, HeisDriver1, HeisDriver2, HeisDriver3};
use motor_driver::{Motor_kontroller, nyMotorKontroller1, nyMotorKontroller2, nyMotorKontroller3};

pub trait Bygning {
    fn hent_heis_driver(&self) -> Box<HeisDriver>;
    fn hent_motor_kontroller(&self) -> Box<MotorKontroller>;
    fn hent_etasje_høyde(&self) -> Vec<f64>;
    fn hent_heis_vekt(&self) -> f64;
    fn clone(&self) -> Box<Bygning>;
    fn serialize(&self) -> u64;
}

pub fn deserialize(n: f64) -> Box<Bygning> {
    if n == 1 {
        Box::new(Bygning1)
    } else if n == 2 {
        Box::new(Bygning2)
    } else {
        Box::new(Bygning3)
    }
}

pub fn hentHeisEtasje(etasjeHøyde: Vec<f64>, høyde: u64) -> f64 {
    let mut c = 0.0;
    for (ei, eh) in etasjeHøyde.iter().enumerate() {
        c += eh;
        if høyde <= c {
            return (ei as u64)
        }
    }
    (etasjeHøyde.len() -1) as u64
}

pub fn hentKumulativEtasjeHøyde(høyde: Vec<f64>, etasje: u64) -> f64 {
    høyde.iter().take(etasje as usize).sum()
}

pub struct Bygning1;

impl Bygning for Bygning1 {
    fn hent_heis_driver(&self) -> Box<HeisDriver> {
        Box::new(HeisDriver1)
    }

    fn hent_motor_kontroller(&self) -> Box<MotorKontroller> {
        nyMotorKontroller1()
    }

    fn hent_etasje_høyde(&self) -> Vec<f64> {
        vec![8.0, 4.0, 4.0, 4.0, 4.0]
    }

    fn hent_heis_vekt(&self) -> f64 {
        1200.0
    }

    fn clone(&self) -> Box<Bygning> {
        Box::new(Bygning1)
    }

    fn serialize(&self) -> u64 {
        1
    }
}
