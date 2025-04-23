use bygninger::{Bygning, hentHeisEtasje};
use fysikk::{HeisStatus};
use std::fs::File;
use std::io::{self, Read, Write};
use std::io::prelude::*;
use termion;
use termion::{clear, cursor, style};
use termion::raw;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use serde_json;

pub trait DataRegistreringer {
    fn init(&mut self, esp: Box<Bygning>, est: HeisStatus);
    fn registrerer(&mut self, est: HeisStatus, dst: u64);
    fn samendrag(&mut self);
}

struct enkelDataRegistrerer<W: Write> {
    esp: Box<Bygning>,
    termbredde: u64,
    termhøyde: u64,
    stdout: raw::RawTerminal<W>,
    log: File,
    registrer_lokasjon: Vec<f64>,
    registrer_hastighet: Vec<f64>,
    registrer_akselerasjon: Vec<f64>,
    registrer_styrke: Vec<u64>,
}

pub fn nyEnkelDataRegistrerer(esp: Box<Bygning>) -> Box<DataRegistreringer> {
    let termsize = termion::treminal_size().ok();
    Box::new(enkelDataRegistrerer {
        esp: esp.clone(),
        termbredde: termsize.map(|(w, _)| w -2).expect("termbredde") as u64,
        termhøyde: termsize.map(|(h, _)| h -2).expect("termhøyde") as u64,
        stdout: io::stdout().into_raw_mode().unwrap(),
        log: File::create("simulator.log").expect("logg fil"),
        registrer_lokasjon: Vec::new(),
        registrer_hastighet: Vec::new(),
        registrer_akselerasjon: Vec::new(),
        registrer_styrke: Vec::new(),
    })
}

impl<W: Write> DataRegistreringer for enkelDataRegistrerer<W> {
    fn init(&mut self, esp: Box<Bygning>, est: HeisStatus) {
        self.esp = esp.clone();
        self.log.write_all(serde_json::to_string(&esp.serialize()).unwrap().as_bytes()).expect("skrive til logg");
        self.log.write_all(b"\n").expect("skrive til logg");
    }

    fn registrere(&mut self, est: HeisStatus, dst: u64) {
        
    }
}