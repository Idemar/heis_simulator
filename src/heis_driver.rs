use libc::c_int;

#[link(navn = "heis1")]
unsafe extern {
    pub fn heis1_etasje_foresporsel() -> c_int;
}

#[link(navn = "heis2")]
unsafe extern {
    pub fn heis2_etasje_foresporsel() -> c_int;
}

#[link(navn = "heis3")]
unsafe extern {
    pub fn heis3_etasje_foresporsel() -> c_int;
}

pub trait HeisDriver {
    fn etasje_foresporsel(&self) -> Option<u64>;
}

pub struct HeisDriver1;

impl HeisDriver for HeisDriver1 {
    fn etasje_foresporsel(&self) -> Option<u64> {
        unsafe {
            let foresporsel = heis1_etasje_foresporsel();
            if foresporsel > 0 {
                Some(foresporsel as u64)
            } else {
                None
            }
        }
    }
}

pub struct HeisDriver2;

impl HeisDriver for HeisDriver2 {
    fn etasje_foresporsel(&self) -> Option<u64> {
        unsafe {
            let foresporsel = heis2_etasje_foresporsel();
            if foresporsel > 0 {
                Some(foresporsel as u64)
            } else {
                None
            }
        }
    }
}

pub struct HeisDriver3;

impl HeisDriver for HeisDriver3 {
    fn etasje_foresporsel(&self) -> Option<u64> {
        unsafe {
            let foresporsel = heis3_etasje_foresporsel();
            if foresporsel > 0 {
                Some(foresporsel as u64)
            } else {
                None
            }
        }
    }
}