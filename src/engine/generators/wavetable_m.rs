use wide::f32x8;

use crate::engine::{chunk::ChunkM, Globals};

pub trait WTGenM {
    fn sample(&self, x: f32, wt_pos: f32) -> f32;
}

pub struct WavetableM<Gen: WTGenM> {
    globals: Globals,
    gen: Gen,  // from 0.0 to 1.0

    pub frequency: f32,
    pub volume: f32,
    pub wt_pos: f32,

    x: f32,
}

impl<Gen: WTGenM> WavetableM<Gen> {
    pub fn new(globals: Globals, gen: Gen, frequency: f32, volume: f32) -> WavetableM<Gen> {
        return WavetableM {
            globals,
            gen,
            frequency,
            volume,
            wt_pos: 0.0,
            x: 0.0,
        }
    }

    fn next(&mut self) -> f32 {
        let x = self.x;
        self.x = (self.x + self.frequency/self.globals.sample_rate as f32) % 1.0;
        self.gen.sample(x, self.wt_pos) * self.volume
    }

    fn next8(&mut self) -> f32x8 {
        f32x8::new([
            self.next(), self.next(), self.next(), self.next(), 
            self.next(), self.next(), self.next(), self.next(), 
        ])
    }

    pub fn render(&mut self, chunk: &mut ChunkM) {
        for i in 0..chunk.c.len() {
            chunk.c[i] += self.next8();
        }
    }
}