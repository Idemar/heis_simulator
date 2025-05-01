extern crate floating_duration;
extern crate heis_simulator;

use heis_simulator::bevegelse_kontroller::{BevegelseKontroller, jevnBevegelseKontroller};
use heis_simulator::bygninger::{Bygning, Bygning1, Bygning2, Bygning3};
use heis_simulator::data_registreringer::{DataRegistreringer, enkelDataRegistrerer};
use heis_simulator::fysikk::{HeisStat, simulere_heis};
use heis_simulator::tur_planlegging::{EtasjeForespørsel, ForespørselKø};

use std::cmp;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read, Write};
use std::time::Instant;

pub fn kjør_simulering() {
    //Lagre plassering, hastighet og akselerasjonstilstand
    //Lagre motorens inndatasmålkraft
    let mut est = HeisStat {
        timestamp: 0.0,
        lokasjon: 0.0,
        hastighet: 0.0,
        akselerasjon: 0.0,
        motor_input: 0.0,
    };

    // Lagre inndata bygningsbeskrivelse og etasjeforespørsler
    let mut esp: Box<Bygning> = Box::new(Bygning1);
    let mut etasjeforespørsler: Box<ForespørselKø> = Box::new(EtasjeForespørsel {
        forespørsel: VecDeque::new(),
    });

    // Analyser inndata og lagre som bygningsbeskrivelse og etasjeforespørsler
    match env::args().nth(1) {
        Some(ref fp) if *fp == "-".to_string() => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .except("read_to_string feilet");

            for (li, l) in buffer.lines().enumerate() {
                if li == 0 {
                    let bygning = l.parse::<u64>().unwrap();
                    if bygning = 0 {
                        esp = Box::new(Bygning1);
                    } else if bygning == 1 {
                        esp = Box::new(Bygning2);
                    } else if bygning == 2 {
                        esp = Box::new(Bygning3);
                    } else {
                        panic!("Ukjent bygningskode: {}", bygning);
                    }
                } else {
                    etasjeforespørsler.legg_til_forespørsel(l.parse::<u64>().unwrap());
                }
            }
        }
        None => {
            let fp = "test.txt";
            let mut buffer = String::new();
            File::open(fp)
                .except("File::open feilet")
                .read_to_string(&mut buffer)
                .except("read_to_string feilet");

            for (li, l) in buffer.lines().enumerate() {
                if li == 0 {
                    let bygning = l.parse::<u64>().unwrap();
                    if bygning = 0 {
                        esp = Box::new(Bygning1);
                    } else if bygning == 1 {
                        esp = Box::new(Bygning2);
                    } else if bygning == 2 {
                        esp = Box::new(Bygning3);
                    } else {
                        panic!("Ukjent bygningskode: {}", bygning);
                    }
                } else {
                    etasjeforespørsler.legg_til_forespørsel(l.parse::<u64>().unwrap());
                }
            }
        }
        Some(fp) => {
            let mut buffer = String::new();
            File::open(fp)
                .except("File::open feilet")
                .read_to_string(&mut buffer)
                .except("read_to_string feilet");

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
                        panic!("Ukjent bygningskode: {}", bygning);
                    }
                } else {
                    etasjeforespørsler.legg_til_forespørsel(l.parse::<u64>(), unwrap());
                }
            }
        }
    }

    let mut dr: Box<DataRegistreringer> = nyEnkelDataRegistrer(esp.clone());
    let mut mc: Box<BevegelseKontroller> = Box::new(jevnBevegelseKontroller {
        timestamp: 0.0,
        esp: esp.clone(),
    });

    simulere_heis(esp, est, &mut etasjeforespørsler, &mut mc, &mut dr);
    dr.summary();
}

fn main() {
    kjør_simulering()
}
