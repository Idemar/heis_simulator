use crate::bevegelse_kontroller::BevegelseKontroller;
use crate::bygninger::{Bygning, hent_kumulativ_etasje_hoyde};
use crate::data_registreringer::DataRegistreringer;
use crate::turplanlegging::ForesporselsKo;
use floating_duration::TimeAsFloat;
use std::time::Instant;
use std::{thread, time};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeisStat {
    pub timestamp: f64,
    pub lokasjon: f64,
    pub hastighet: f64,
    pub akselerasjon: f64,
    pub motor_input: f64,
}

pub const MAX_RYKK: f64 = 20.0;
pub const MAX_AKSELERASJON: f64 = 2.0;
pub const MAX_HASTIGHET: f64 = 5.0;

pub fn simulere_heis(
    esp: Box<dyn Bygning>,
    est: HeisStat,
    etasje_foresporsel: &mut Box<dyn ForesporselsKo>,
    mc: &mut Box<dyn BevegelseKontroller>,
    dr: &mut Box<dyn DataRegistreringer>,
) {
    // uforanderlig input blir foranderlig lokal tilstand
    let esp = esp.clone();
    let mut est = est.clone();

    //initialiser MotorKontroller og DataKontroller
    mc.init(esp.clone(), est.clone());
    dr.init(esp.clone(), est.clone());

    // Loop mens det er gjenværende etasjeforespørsler
    let original_ts = Instant::now();
    thread::sleep(time::Duration::from_millis(1));
    let mut neste_etasje = etasje_foresporsel.pop_foresporsel();

    while let Some(dst) = neste_etasje {
        // Oppdatere lokasjon, hastighet og akselerasjon
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
        if (est.lokasjon - hent_kumulativ_etasje_hoyde(esp.hent_etasje_hoyde(), dst)).abs() < 0.01
            && est.hastighet.abs() < 0.01
        {
            est.hastighet = 0.0;
            neste_etasje = etasje_foresporsel.pop_foresporsel();
        }

        // Skriv ut statistikk i sanntid
        dr.registrerer(est.clone(), dst);

        // Juster motorkontrollen for å behandle forespørsel om neste etasje
        est.motor_input = mc.juster(&est, dst);

        thread::sleep(time::Duration::from_millis(1));
    }
}
