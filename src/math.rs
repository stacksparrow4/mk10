use num::Zero;

use crate::types::Frac;

pub fn frac_pow(a: Frac, b: Frac) -> Option<Frac> {
    if b.is_integer() {
        if b.is_zero() {
            Some(Frac::from(1))
        } else if a.is_zero() {
            Some(Frac::from(0))
        } else {
            let mut b = b.to_integer();
            let a_flipped = if b < 0 {
                b = -b;
                a.recip()
            } else {
                a
            };
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
