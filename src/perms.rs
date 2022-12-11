use std::iter;

use crate::types::{Frac, RecurseNum, RecurseState};

pub fn recurse_join_perms(data: &[i32]) -> Vec<Vec<i32>> {
    if data.len() > 1 {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let next = recurse_join_perms(&data[1..]);

        ret.extend(
            next.iter()
                .map(|d| iter::once(data[0]).chain(d.iter().map(|x| *x)).collect()),
        );
        ret.extend(next.iter().map(|d| {
            iter::once(i32::from_str_radix(&format!("{}{}", data[0], d[0]), 10).unwrap())
                .chain(d.iter().skip(1).map(|x| *x))
                .collect()
        }));

        ret
    } else {
        vec![Vec::from(data)]
    }
}

pub fn recurse_signs(data: &[i32]) -> Vec<Vec<i32>> {
    if data.len() > 1 {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let next = recurse_signs(&data[1..]);

        ret.extend(
            next.iter()
                .map(|d| iter::once(data[0]).chain(d.iter().map(|x| *x)).collect()),
        );
        ret.extend(
            next.iter()
                .map(|d| iter::once(-data[0]).chain(d.iter().map(|x| *x)).collect()),
        );

        ret
    } else if data.len() == 1 {
        vec![vec![data[0]], vec![-data[0]]]
    } else {
        Vec::new()
    }
}

pub fn gen_perms(data: &[i32]) -> Vec<RecurseState> {
    assert!(data.len() > 0);

    recurse_join_perms(data)
        .into_iter()
        .flat_map(|d| recurse_signs(&d))
        .map(|d| {
            d.iter()
                .map(|val| RecurseNum {
                    value: Frac::from(*val),
                    repr: format!("{}", *val),
                })
                .collect()
        })
        .collect()
}
