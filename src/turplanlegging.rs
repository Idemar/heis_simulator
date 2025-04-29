use std::collections::VecDeque;

pub struct EtasjeForespørsel {
    pub forespørsel: VecDeque<u64>,
}

pub trait ForespørselsKø {
    fn legg_til_forespørsel(&mut self, fors: u64);
    fn legg_til_forespørseler(&mut self, forser: &Vec<u64>);
    fn pop_forespørsel(&mut self) -> Option<u64>;
}

impl ForespørselsKø for EtasjeForespørsel {
    fn legg_til_forespørsel(&mut self, fors: u64) {
        self.forespørsel.push_back(fors);
    }

    fn legg_til_forespørseler(&mut self, forser: &Vec<u64>) {
        for fors in forser {
            self.forespørsel.push_back(*fors);
        }
    }

    fn pop_forespørsel(&mut self) -> Option<u64> {
        self.forespørsel.pop_front()
    }
}
