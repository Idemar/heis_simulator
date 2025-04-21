use libc::c_int;

#[link(navn: "heis1")]
extern {
    pub fn heis1_etasje_forespørsel() -> c_int;
}

#[link(navn: "heis2")]
extern {
    pub fn heis2_etasje_forespørsel() -> c_int;
}

#[link(navn: "heis3")]
extern {
    pub fn heis3_etasje_forespørsel() -> c_int;
}

pub trait HeisDriver {
    fn etasje_forespørsel(&self) -> Option<u64>;
}

pub struct HeisDriver1;

impl HeisDriver for HeisDriver1 {
    fn etasje_forespørsel(&self) -> Option<u64> {
        unsafe {
            let forespørsel = heis1_etasje_forespørsel();
            if forespørsel > 0 {
                Some(forespørsel as u64)
            } else {
                None
            }
        }
    }
}

pub struct HeisDriver2;

impl HeisDriver for HeisDriver2 {
    fn etasje_forespørsel(&self) -> Option<u64> {
        unsafe {
            let forespørsel = heis2_etasje_forespørsel();
            if forespørsel > 0 {
                Some(forespørsel as u64)
            } else {
                None
            }
        }
    }
}

pub struct HeisDriver3;

impl HeisDriver for HeisDriver3 {
    fn etasje_forespørsel(&self) -> Option<u64> {
        unsafe {
            let forespørsel = heis3_etasje_forespørsel();
            if forespørsel > 0 {
                Some(forespørsel as u64)
            } else {
                None
            }
        }
    }
}