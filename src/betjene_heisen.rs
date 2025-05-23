extern crate floating_duration;
extern crate heis_simulator;

use heis_simulator::bevegelse_kontroller::{BevegelseKontroller, JevnBevegelseKontroller};
use heis_simulator::bygninger::{
    Bygning, Bygning1, Bygning2, Bygning3, hent_kumulativ_etasje_hoyde,
};
use heis_simulator::fysikk::HeisStat;
use heis_simulator::turplanlegging::{EtasjeForesporsel, ForesporselsKo};

use floating_duration::TimeAsFloat;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;
use std::{thread, time};

pub fn kjør_operatør() {
    // Lagre plassering, hastighet og akselerasjonstilstand
    // Lagre motorens inndatasmålkraft
    let mut est = HeisStat {
        timestamp: 0.0,
        lokasjon: 0.0,
        hastighet: 0.0,
        akselerasjon: 0.0,
        motor_input: 0.0,
    };

    // Lagre inndata bygningsbeskrivelse og etasjeforespørsler
    let mut esp: Box<dyn Bygning> = Box::new(Bygning1);
    let mut etasjeforesporsler: Box<dyn ForesporselsKo> = Box::new(EtasjeForesporsel {
        foresporsel: VecDeque::new(),
    });

    // Analyser inndata og lagre som bygningsbeskrivelse og etasjeforespørsler
    match env::args().nth(1) {
        Some(ref fp) if *fp == "-".to_string() => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("read_to_string feilet");

            for (li, l) in buffer.lines().enumerate() {
                if li == 0 {
                    let bygning = l.parse::<u64>().unwrap();
                    if bygning == 0 {
                        esp = Box::new(Bygning1);
                    } else if bygning == 1 {
                        esp = Box::new(Bygning2);
                    } else if bygning == 2 {
                        esp = Box::new(Bygning3);
                    } else {
                        panic!("Ukjent bygingskode: {}", bygning);
                    }
                } else {
                    etasjeforesporsler.legg_til_foresporsel(l.parse::<u64>().unwrap());
                }
            }
        }
        None => {
            let fp = "test.txt";
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open feilet")
                .read_to_string(&mut buffer)
                .expect("read_to_string feilet");

            for (li, l) in buffer.lines().enumerate() {
                if li == 0 {
                    let bygning = l.parse::<u64>().unwrap();
                    if bygning == 0 {
                        esp = Box::new(Bygning1);
                    } else if bygning == 1 {
                        esp = Box::new(Bygning2);
                    } else if bygning == 2 {
                        esp = Box::new(Bygning3);
                    } else {
                        panic!("Ukjent bygingskode: {}", bygning);
                    }
                } else {
                    etasjeforesporsler.legg_til_foresporsel(l.parse::<u64>().unwrap());
                }
            }
        }
        Some(fp) => {
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open feilet")
                .read_to_string(&mut buffer)
                .expect("read_to_string feilet");

            for (li, l) in buffer.lines().enumerate() {
                if li == 0 {
                    let bygning = l.parse::<u64>().unwrap();
                    if bygning == 0 {
                        esp = Box::new(Bygning1);
                    } else if bygning == 1 {
                        esp = Box::new(Bygning2);
                    } else if bygning == 2 {
                        esp = Box::new(Bygning3);
                    } else {
                        panic!("Ukjent bygingskode: {}", bygning);
                    }
                } else {
                    etasjeforesporsler.legg_til_foresporsel(l.parse::<u64>().unwrap());
                }
            }
        }
    }

    let mut mc: Box<dyn BevegelseKontroller> = Box::new(JevnBevegelseKontroller {
        timestamp: 0.0,
        esp: esp.clone(),
    });

    //initialiser MotorKontroller og DataKontroller
    mc.init(esp.clone(), est.clone());

    // Loop mens det er gjenværende etasjeforespørsler
    let original_ts = Instant::now();
    thread::sleep(time::Duration::from_millis(1));
    let mut neste_etasje = etasjeforesporsler.pop_foresporsel();

    loop {
        if let Some(dst) = neste_etasje {
            // oppdater lokasjon, hastighet og akselerasjon
            let now = Instant::now();
            let ts = now.duration_since(original_ts).as_fractional_secs();
            let dt = ts - est.timestamp;
            est.timestamp = ts;

            est.lokasjon += est.hastighet * dt;
            est.hastighet += est.akselerasjon * dt;
            est.akselerasjon = {
                let f = est.motor_input;
                let m = esp.hent_heis_vekt();
                -9.8 + f / m
            };

            // Hvis forespørselen om neste etasje i køen er oppfylt, fjern den fra køen
            if (est.lokasjon - hent_kumulativ_etasje_hoyde(esp.hent_etasje_hoyde(), dst)).abs()
                < 0.01
                && est.hastighet.abs() < 0.01
            {
                est.hastighet = 0.0;
                neste_etasje = etasjeforesporsler.pop_foresporsel();
            }

            // Juster motorkontrollen for å behandle forespørsel om neste etasje
            est.motor_input = mc.juster(&est, dst);

            // Juster motor_input
            esp.hent_motor_kontroller().juster_motor(est.motor_input);

            thread::sleep(time::Duration::from_millis(1));
        } else {
            // Juster motoren slik at den ikke beveger seg
            esp.hent_motor_kontroller().juster_motor(0.0);
        }

        // sjekk for dynamiske etasjeforespørsler
        if let Some(dst) = esp.hent_heis_driver().etasje_foresporsel() {
            etasjeforesporsler.legg_til_foresporsel(dst);
        }
    }
}

fn main() {
    kjør_operatør()
}
