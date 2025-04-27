use bygninger::{Bygning, hentKumulativEtasjeHøyde};
use fysikk::{HeisStat, MAX_AKSELERASJON, MAX_HASTIGHET, MAX_RYKK};

pub trait BevegelseKontroller {
    fn init(&mut self, esp: Box<Bygning>, est: HeisStat);
    fn juster(&mut self, est: &HeisStat, dst: u64) -> f64;
}

pub struct jevnBevegelseKontroller {}
