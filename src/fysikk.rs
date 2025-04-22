use bevegelse_kontroller::{BevegelseKontroller};
use bygninger::{Bygning, hentKumulativEtasjeHøyde};
use data_registrering::{DataRegistrering};
use floating_duration::{TimeAsFloat, TimeFormat};
use std::time::Instant;
use std::{Thread, time};
use tur_planlegger::{ForespørselKø};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeisStatus {
    pub timestamp: f64,
    pub lokasjon: f64,
    pub hastighet: f64,
    pub akselerasjon: f64,
    pub motor_input: f64,
}

pub const MAX_RYKK: f64 = 20.0;
pub const MAX_AKSELERASJON: f64 = 2.0;
pub const MAX_HASTIGHET: f64 = 5.0;

pub simulere_heis(hby: Box<Bygning>, hst: HeisStatus, etasje_forespørsel: &mut Box<ForespørselKø>, bk: &mut Box<BevegelseKontroller>, dr: &mut Box<DataRegistrering>) {
    
    // uforanderlig input blir foranderlig lokal tilstand
    let mut esp = esp.clone();
    let mut est = est.clone();

    //initialiser MotorKontroller og DataKontroller
    mc.init(esp.clone(, est.clone()));
    dr.init(esp.clone(), est.clone());

    // Loop mens det er gjenværende etasjeforespørsler
    let original_ts = Instant::now();
    thread::sleep(time::Duration::from_millis(1));
    let mut neste_etasje = etasje_forespørsel.pop_request();

    while let Some(dst) = neste_etasje {

        // Oppdatere lokasjon, hastighet og akselerasjon
        let now = Instant::now();
        let ts = now.duration_since(original_ts).as_fractional_seconds();
        let dt = ts - est.timestamp;
        est.timestamp = ts;

        est.lokasjon = est.lokasjon + est.hastighet * dt;
        est.hastighet = est.hastighet + est.akselerasjon * dt;
        est.akselerasjon = {
            let F = est.motor_input;
            let m = esp.hent_heis_vekt();
            -9.8 + F/m
        };

        // Hvis forespørselen om neste etasje i køen er oppfylt, fjern den fra køen
        if (est.lokasjon - hentKumulativEtasjeHøyde(esp.hent_etasje_høyde(), dst)).abs() < 0.01 && est.hastighet.abs() < 0.01 {
            est.hastighet = 0.0;
            neste_etasje = etasje_forespørsel.pop_request();
        }

        // Skriv ut statistikk i sanntid
        dr.record(est.clone(), dst);

        // Juster motorkontrollen for å behandle forespørsel om neste etasje
        est.motor_input = mc.juster(&est, dst);

        thread::sleep(time::Duration::from_millis(1));

    }
}