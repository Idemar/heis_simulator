use bevegelse_kontroller::BevegelseKontroller;
use bygninger::{Bygning, hentKumulativEtasjeHøyde};
use data_registrering::DataRegistrering;
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
    let mut hby = hby.clone();
    let mut hst = hst.clone();

}