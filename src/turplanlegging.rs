use std::collections::VecDeque;

pub struct EtasjeForesporsel {
    pub foresporsel: VecDeque<u64>,
}

pub trait ForesporselsKo {
    fn legg_til_foresporsel(&mut self, fors: u64);
    fn legg_til_foresporseler(&mut self, forser: &Vec<u64>);
    fn pop_foresporsel(&mut self) -> Option<u64>;
}

impl ForesporselsKo for EtasjeForesporsel {
    fn legg_til_foresporsel(&mut self, fors: u64) {
        self.foresporsel.push_back(fors);
    }

    fn legg_til_foresporseler(&mut self, forser: &Vec<u64>) {
        for fors in forser {
            self.foresporsel.push_back(*fors);
        }
    }

    fn pop_foresporsel(&mut self) -> Option<u64> {
        self.foresporsel.pop_front()
    }
}
