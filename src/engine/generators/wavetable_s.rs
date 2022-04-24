use wide::f32x8;

use crate::engine::{chunk::ChunkS, Globals};

pub trait WTGenS {
    fn sample(&self, x: f32, wt_pos: f32) -> (f32, f32);
}

pub struct WavetableS<Gen: WTGenS> {
    globals: Globals,
    gen: Gen,  // from 0.0 to 1.0

    pub frequency: f32,
    pub volume: f32,
    pub wt_pos: f32,

    x: f32,
}

impl<Gen: WTGenS> WavetableS<Gen> {
    pub fn new(globals: Globals, gen: Gen, frequency: f32, volume: f32) -> WavetableS<Gen> {
        return WavetableS {
            globals,
            gen,
            frequency,
            volume,
            wt_pos: 0.0,
            x: 0.0,
        }
    }

    fn next(&mut self) -> (f32, f32) {
        let x = self.x;
        self.x = (self.x + self.frequency/self.globals.sample_rate as f32) % 1.0;
        let (l, r) = self.gen.sample(x, self.wt_pos);
        (l * self.volume, r * self.volume)
    }

    fn next8(&mut self) -> (f32x8, f32x8) {
        let (l1, r1) = self.next();
        let (l2, r2) = self.next();
        let (l3, r3) = self.next();
        let (l4, r4) = self.next();
        let (l5, r5) = self.next();
        let (l6, r6) = self.next();
        let (l7, r7) = self.next();
        let (l8, r8) = self.next();
        return (
            f32x8::new([l1, l2, l3, l4, l5, l6, l7, l8]),
            f32x8::new([r1, r2, r3, r4, r5, r6, r7, r8]),
        )
    }

    pub fn render(&mut self, chunk: &mut ChunkS) {
        for i in 0..chunk.l.len() {
            let (l, r) = self.next8();
            chunk.l[i] += l;
            chunk.r[i] += r;
        }
    }
}