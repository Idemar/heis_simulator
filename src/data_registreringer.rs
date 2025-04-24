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
        let datum = serde_json::to_string(&(est.clone(), dst)).unwrap();
        self.log.write_all(datum.as_bytes()).expect("skrive stat til logg");
        self.log.write_all(b"\n").expect("skrive stat til logg");

        self.registrer_lokasjon.push(est.lokasjon);
        self.registrer_hastighet.push(est.hastighet);
        self.registrer_akselerasjon.push(est.akselerasjon);
        self.registrer_styrke.push(est.motor_input);

        // Skriv ut statistikk i sanntid
        print!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide):
        let heis_etasje = hentHeisEtasje(self.esp.hentEtasjeHøyde(), est.lokasjon);
        let etasje_teller = self.esp.hentEtasjeHøyde().len() as u64;
        let mut terminal_buffer = vec![' ' as u8; (self.termbredde * self.termhøyde) as usize];

        for ty in 0..etasje_teller {
            terminal_buffer[ (ty * self.termbredde + 0) as usize ] = '[' as u8;
            terminal_buffer[ (ty * selftermbredde + 1) as usize ] =
                if (ty as u64) == ((etasje_teller -1) - heis_etasje) { 'X' as u8}
                else {' ' as u8};
            terminal_buffer[ (ty * self.termbredde + 2) as usize ] = ']' as u8;
            terminal_buffer[ (ty * self.termbredde + self.termbredde - 2) as usize ] = '\r' as u8;
            terminal_buffer[ (ty * self.termbredde + self.termbrerdde - 1) as usize] = '\n' as u8;      }
        }

        let stat = vec![
            format!("Heis i etasje      {}", heis_etasje +1),
            format!("Lokasjon           {:.06}", est.lokasjon),
            format!("Hastighet          {:.06}", est.hastighet),
            format!("Akselerasjon       {:.06}", es.akselerasjon),
            format!("Styrke [opp-ned]   {:.06}", est.motor_input),
        ];

        for sy in 0..stat.len() {
            for (sx, sc) in stat[sy].chars().enumerate() {
                terminal_buffer[ sy * (self.termbredde as usize) + 6 + sx] = sc as u8;
            }
        }

        write!(self.stdout, "{}", String::from_utf8(terminal_buffer).ok().unwrap());
        self.stdout.flush().unwrap();
    }
}