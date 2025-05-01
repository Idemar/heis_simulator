use crate::bygninger::{Bygning, hentKumulativEtasjeHoyde};
use crate::fysikk::{HeisStat, MAX_AKSELERASJON, MAX_HASTIGHET, MAX_RYKK};

pub trait BevegelseKontroller {
    fn init(&mut self, esp: Box<Bygning>, est: HeisStat);
    fn juster(&mut self, est: &HeisStat, dst: u64) -> f64;
}

pub struct JevnBevegelseKontroller {
    pub esp: Box<dyn Bygning>,
    pub timestamp: f64,
}

impl BevegelseKontroller for JevnBevegelseKontroller {
    fn init(&mut self, esp: Box<Bygning>, est: HeisStat) {
        self.esp = esp;
        self.timestamp = est.timestamp;
    }

    fn juster(&mut self, est: &HeisStat, dst: u64) -> f64 {
        // Juster motorkontrollen for å behandle forespørsel om neste etasje

        // det vil ta "t" sekunder å nå maks fra maks
        let t_akselerasjon = MAX_AKSELERASJON / MAX_RYKK;
        let t_hastighet = MAX_HASTIGHET / MAX_AKSELERASJON;

        // det kan ta opptil "d" meter å bremse ned fra nåværende
        let brems_t = if (est.hastighet > 0.0) == (est.akselerasjon > 0.0) {
            // Denne saken overvurderer bevisst "d" for å forhindre "rygging"
            (est.akselerasjon.abs() / MAX_RYKK)
                + (est.hastighet.abs() / (MAX_AKSELERASJON / 2))
                + 2.0 * (MAX_AKSELERASJON / MAX_RYKK)
        } else {
            // uten MAX_RUKK nærmer dette seg uendelig og bremser ned altfor tidlig
            // MAX_RYKK * 1s = akselerasjon i m/s^2
            est.hastighet.abs() / (MAX_RYKK + est.akselerasjon.abs())
        };

        let d = est.hastighet.abs() * brems_t;

        let dst_hoyde = hentKumulativEtasjeHoyde(self.esp.hent_etasje_hoyde(), dst);

        // l = avstand til neste etasje
        let l = (est.lokasjon - dst_hoyde).abs();

        let mal_akselerasjon = {
            // skal vi opp?
            let gar_opp = est.lokasjon < dst_hoyde;

            // tid som har gått siden forrige trekk
            let dt = est.timestamp - self.timestamp;
            self.timestamp = est.timestamp;

            // Ikke overskrid maksimal akselerasjon
            if est.akselerasjon.abs() >= MAX_AKSELERASJON {
                if est.akselerasjon > 0.0 {
                    est.akselerasjon - (dt * MAX_RYKK)
                } else {
                    est.akselerasjon + (dt * MAX_RYKK)
                }

            // Ikke overskrid maksimal hastighet
            } else if est.hastighet.abs() >= MAX_HASTIGHET
                || (est.hastighet + est.akselerasjon * (est.akselerasjon.abs() / MAX_RYKK)).abs()
                    >= MAX_HASTIGHET
            {
                if est.hastighet > 0.0 {
                    est.akselerasjon - (dt * MAX_RYKK)
                } else {
                    est.akselerasjon + (dt * MAX_RYKK)
                }

            // Hvis den er innenfor et komfortabelt retardasjonsområde og beveger seg i riktig retning, deselerer
            } else if l < d && (est.hastighet > 0.0) == gar_opp {
                if gar_opp {
                    est.akselerasjon - (dt * MAX_RYKK)
                } else {
                    est.akselerasjon + (dt * MAX_RYKK)
                }
            // ellers hvis ikke ved topphastighet, akselerer jevnt
            } else {
                if gar_opp {
                    est.akselerasjon + (dt * MAX_RYKK)
                } else {
                    est.akselerasjon - (dt * MAX_RYKK)
                }
            }
        };

        let tyngdekraftsjustert_akselerasjon = mal_akselerasjon + 9.8;
        let mal_styrke = tyngdekraftsjustert_akselerasjon * self.esp.hent_heis_vekt();
        if !mal_styrke.is_finite() {
            //dele på null osv.
            //kan skje hvis tidsdeltaet renner ut
            0.0
        } else {
            mal_styrke
        }
    }
}
