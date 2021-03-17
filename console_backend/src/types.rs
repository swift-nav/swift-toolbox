// use chrono::{DateTime, Utc};
// use serde::Serialize;

// use crate::formatters::*;
// use crate::msg_utils::{GnssMode, InsMode, ProtectionLevel};

use std::borrow::BorrowMut;

use ndarray::{arr0, arr1, arr2, Array, Array0, Array1, Array2, ArrayBase, Axis, concatenate, Dim, OwnedRepr, s, stack};
use ordered_float::OrderedFloat;

use crate::constants::*;

// pub type UtcDateTime = DateTime<Utc>;

pub type Error = std::boxed::Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct TrackingTab {

}

#[derive(Default, Debug)]
pub struct TrackingSignalsTab {
    pub sats: Array2<(f64, OrderedFloat<f64>)>,
    // pub labels: Array1<String>,
}

impl TrackingSignalsTab {
    pub fn new() -> TrackingSignalsTab {
        TrackingSignalsTab {
            sats: Array::from_elem((NUM_SATELLITES, NUM_POINTS), (0.0 as f64, OrderedFloat(0.0 as f64)))
        }
    }
    pub fn append_to_sat(&mut self, sat: usize, ele: &(f64, OrderedFloat<f64>)) {
        
        let mut sat_row_old = self.sats.slice(s![sat,1..]).to_vec();
        sat_row_old.push(*ele);
        self.sats.slice_mut(s![sat,..]).assign(&Array::from(sat_row_old));
        // let sat_row_old = &self.sats.slice(s![sat,1..]);
        // let ele_arr: Array1<(f64, OrderedFloat<f64>)> = arr1(&[*ele]);
        // println!("{:?}", sat_row_old.shape());
        // println!("{:?}", ele_arr.shape());
        // let new_arr = Array1<(f64, OrderedFloat<f64>)> 
        // let new_row: ArrayBase<OwnedRepr<(f64, OrderedFloat<f64>)>, Dim<[usize; 2]>>  = concatenate![Axis(1), sat_row_old, ele_arr];
        // self.sats.slice_mut(s![sat,..]).slice(s![..(NUM_POINTS-1)]).assign(sat_row_old.into());
        // sat_row.slice_mut(s![sat,self.sats.ncols()]).assign(ele);
        // let array1 = arr1(&[1,2,3]);
        // let array2 = arr1(&[3]);
        // let array3 = stack![Axis(1), array1, array2];
        // println!("{:?}", array3);
        let f: Array1<u8> = arr1(&[3 as u8, 4 as u8]);
        // println!("{:?}", sat_row.shape());
    }
}

#[test]
fn test_append_tracking_signals_tab() {
    let mut tst = TrackingSignalsTab::new();
    assert_eq!(tst.sats.shape(), &[NUM_SATELLITES, NUM_POINTS]);
    let ele1 = (64.0 as f64, OrderedFloat(999888.0 as f64));
    tst.append_to_sat(36 as usize, &ele1.clone());
    assert_eq!(&ele1, tst.sats.get((36, NUM_POINTS-1)).unwrap());
}