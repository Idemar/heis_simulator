use crate::heis_driver::{HeisDriver, HeisDriver1, HeisDriver2, HeisDriver3};
use crate::motor_kontroller::{MotorKontroller, nyMotorKontroller1, nyMotorKontroller2, nyMotorKontroller3};

pub trait Bygning {
    fn hent_heis_driver(&self) -> Box<dyn HeisDriver>;
    fn hent_motor_kontroller(&self) -> Box<dyn MotorKontroller>;
    fn hent_etasje_hoyde(&self) -> Vec<f64>;
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

pub fn hentHeisEtasje(etasjeHoyde: Vec<f64>, hoyde: u64) -> f64 {
    let mut c = 0.0;
    for (ei, eh) in etasjeHoyde.iter().enumerate() {
        c += eh;
        if hoyde <= c {
            return ei as u64
        }
    }
    (etasjeHoyde.len() -1) as u64
}

pub fn hentKumulativEtasjeHoyde(hoyde: Vec<f64>, etasje: u64) -> f64 {
    hoyde.iter().take(etasje as usize).sum()
}

pub struct Bygning1;

impl Bygning for Bygning1 {
    fn hent_heis_driver(&self) -> Box<dyn HeisDriver> {
        Box::new(HeisDriver1)
    }

    fn hent_motor_kontroller(&self) -> Box<dyn MotorKontroller> {
        nyMotorKontroller1()
    }

    fn hent_etasje_hoyde(&self) -> Vec<f64> {
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

pub struct Bygning2;

impl Bygning for Bygning2 {
    fn hent_heis_driver(&self) -> Box<dyn HeisDriver> {
        Box::new(HeisDriver2)
    }

    fn hent_motor_kontroller(&self) -> Box<dyn MotorKontroller> {
        nyMotorKontroller2()
    }

    fn hent_etasje_hoyde(&self) -> Vec<f64> {
        vec![5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0]
    }

    fn hent_heis_vekt(&self) -> f64 {
        1350.0
    }

    fn clone(&self) -> Box<Bygning> {
        Box::new(Bygning2)
    }

    fn serialize(&self) -> u64 {
        2
    }
}

pub struct Bygning3;

impl Bygning for Bygning3 {
    fn hent_heis_driver(&self) -> Box<dyn HeisDriver> {
        Box::new(HeisDriver3)
    }

    fn hent_motor_kontroller(&self) -> Box<dyn MotorKontroller> {
        nyMotorKontroller3()
    }

    fn hent_etasje_hoyde(&self) -> Vec<f64> {
        vec![6.0, 4.0, 4.0, 4.0]
    }

    fn hent_heis_vekt(&self) -> f64 {
        1500.0
    }
    fn clone(&self) -> Box<Bygning> {
        Box::new(Bygning3)
    }
    fn serialize(&self) -> u64 {
        3
    }
}