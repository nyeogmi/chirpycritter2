use std::f32::consts::PI;

use super::wavetable_m::WTGenM;

pub struct Sine {}
impl WTGenM for Sine {
    fn sample(&self, x: f32, _wt_pos: f32) -> f32 {
        return (2.0 * PI * x).sin()
    }
}

pub struct Square {}
impl WTGenM for Square {
    fn sample(&self, x: f32, _wt_pos: f32) -> f32 {
        if x < 0.5 {
            return -1.0;
        } else {
            return 1.0;
        }
    }
}