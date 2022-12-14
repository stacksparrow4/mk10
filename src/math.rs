use num::Zero;

use crate::types::Frac;

fn try_recip(f: Frac) -> Option<Frac> {
    if f.numer() == &0 {
        None
    } else if f.numer() > &0 {
        Some(Frac::new_raw(*f.denom(), *f.numer()))
    } else {
        let new_numer = f.denom().checked_neg();
        let new_denom = f.numer().checked_neg();
        if new_numer.is_none() || new_denom.is_none() {
            None
        } else {
            Some(Frac::new_raw(new_numer.unwrap(), new_denom.unwrap()))
        }
    }
}

pub fn frac_pow(a: Frac, b: Frac) -> Option<Frac> {
    if b.is_integer() {
        if b.is_zero() {
            Some(Frac::from(1))
        } else if a.is_zero() {
            Some(Frac::from(0))
        } else {
            let mut b = b.to_integer();
            let mut a_flipped = a;
            if b < 0 {
                let b_checked = b.checked_neg();
                if b_checked.is_none() {
                    return None;
                }
                b = b_checked.unwrap();

                let a_flipped_res = try_recip(a);
                if a_flipped_res.is_none() {
                    return None;
                }
                a_flipped = a_flipped_res.unwrap();
            }
            if let Ok(b) = b.try_into() {
                if let Some(numer) = a_flipped.numer().checked_pow(b) {
                    if let Some(denom) = a_flipped.denom().checked_pow(b) {
                        return Some(Frac::new(numer, denom));
                    }
                }
            }
            None
        }
    } else {
        None
    }
}
