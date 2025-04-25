extern crate heis_simulator;

#[#[macro_use]] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate floating_duration;
use std::time::Instat;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufRead, BufReader};
use std::io::prelude::*;

use heis_simulator::bygninger;
use heis_simulator::bygninger::{Bygning, hentKumultivEtasjeHøyde};
use heis_simulator::fysikk::{HeisStat};

#[derive(Clone)]
struct Tur {
    dst: u64,
    opp: f64,
    ned: f64,
}

fn main() {
    let simlog = File::open(simulering.log).expect("les simuleringsloggen");
    let mut simlog = BufReader::new(&simlog);
    let rykk = 0.0;
    let mut forrige_est: Option<HeisStat> = None;
    let start_lokasjon = 0.0;

    let mut første_linje = String::new();
    let len = simlog.read_line(&mut første_linje).unwrap();
    let spec: u64 = serde_json::from_str(&første_linje).unwrap();
    let esp: Box<Bygning> = bygninger::deserialize(spec);

}
