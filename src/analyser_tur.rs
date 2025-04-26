extern crate heis_simulator;

#[macro_use]
extern crate serde_derive;
extern crate floating_duration;
extern crate serde;
extern crate serde_json;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::time::Instat;

use heis_simulator::bygninger;
use heis_simulator::bygninger::{Bygning, hentKumultivEtasjeHøyde};
use heis_simulator::fysikk::HeisStat;

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
    for linje in simlog.lines() {
        let l = linje.unwrap();
        let (est, dst): (HeisStat, u64) = serde_json::from_str(&l).unwrap();
        let dl = dst_timing.len();
        if dst_timing.len() == 0 || dst_timing[dl - 1].dst != dst {
            dst_timing.push(Tur {
                dst: dst,
                opp: 0.0,
                ned: 0.0,
            });
        }

        if let Some(forrige_est) = forrige_est {
            let dt = est.timestamp - forrige_est.timestamp;
            if est.hastighet > 0.0 {
                dst_timing[dl - 1].opp += dt;
            } else {
                dst_timing[dl - 1].ned += dt;
            }

            let da = (est.akselerasjon - forrige_est.akselerasjon).abs();
            rykk = (rykk * (1.0 - dt)) + (da * dt);
            if rykk.abs() > 0.22 {
                panic!("rykk er utenfor akseptable grenser: {} {:?}", rykk, est)
            }
        } else {
            start_lokasjon = est.lokasjon;
        }

        if est.akselerasjon.abs() > 2.2 {
            panic!("akselerasjonen er utenfor akseptable grenser: {:?}", est)
        }

        if est.hastighet.abs() > 5.5 {
            panic!("Hastigheten er utenfor akseptable grenser: {:?}", est)
        }

        forrige_est = Some(est)
    }
}
