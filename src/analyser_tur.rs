extern crate heis_simulator;

extern crate floating_duration;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader};

use heis_simulator::bygninger;
use heis_simulator::bygninger::{Bygning, hent_kumulativ_etasje_hoyde};
use heis_simulator::fysikk::HeisStat;

#[derive(Clone)]
struct Tur {
    dst: u64,
    opp: f64,
    ned: f64,
}

fn main() {
    let simlog = File::open("simulation.log").expect("les simuleringsloggen");
    let mut simlog = BufReader::new(&simlog);
    let mut rykk = 0.0;
    let mut forrige_est: Option<HeisStat> = None;
    let mut dst_timing: Vec<Tur> = Vec::new();
    let mut start_lokasjon = 0.0;

    let mut første_linje = String::new();
    let len = simlog.read_line(&mut første_linje).unwrap();
    let spec: u64 = serde_json::from_str(&første_linje).unwrap();
    let esp: Box<dyn Bygning> = bygninger::deserialize(spec as f64);
    for linje in simlog.lines() {
        let l = linje.unwrap();
        let (est, dst): (HeisStat, u64) = serde_json::from_str(&l).unwrap();
        let dl = dst_timing.len();
        if dst_timing.len() == 0 || dst_timing[dl - 1].dst != dst {
            dst_timing.push(Tur {
                dst,
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

    // Heisen skal ikke rygge
    let mut total_tid = 0.0;
    let mut total_direkte = 0.0;
    for tur in dst_timing.clone() {
        total_tid += tur.opp + tur.ned;
        if tur.opp > tur.ned {
            total_direkte += tur.opp;
        } else {
            total_direkte += tur.ned
        }
    }

    if (total_direkte / total_tid) < 0.9 {
        panic!("Heiskrasj er for vanlig: {}", total_direkte / total_tid)
    }

    // turene bør fullføres innenfor 20 % av den teoretiske grensen
    let MAX_RYKK = 0.2;
    let MAX_AKSELERASJON = 2.0;
    let MAX_HASTIGHET = 5.0;

    let mut tur_start_lokasjon = start_lokasjon;
    let mut teoretisk_tid = 0.0;
    let etasje_hoyde = esp.hent_etasje_hoyde();
    for tur in dst_timing.clone() {
        let neste_etasje = hent_kumulativ_etasje_hoyde(etasje_hoyde.clone(), tur.dst);
        let d = (tur_start_lokasjon - neste_etasje).abs();
        teoretisk_tid += 2.0 * (MAX_AKSELERASJON / MAX_RYKK)
            + 2.0 * (MAX_RYKK / MAX_AKSELERASJON)
            + d / MAX_HASTIGHET;

        tur_start_lokasjon = neste_etasje
    }

    if total_tid > (teoretisk_tid * 1.2) {
        panic!(
            "heisen går for saktere {} {}",
            total_tid,
            teoretisk_tid * 1.2
        )
    }

    println!("Alle simuleringskontroller består");
}
