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
}
