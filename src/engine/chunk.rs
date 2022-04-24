use wide::f32x8;

#[derive(Debug)]
pub struct ChunkS { 
    // 512 samples: 44100 samples per second; can track changes at 86hz
    pub l: [f32x8; 64],
    pub r: [f32x8; 64],
}

impl ChunkS {
    pub fn new() -> ChunkS {
        ChunkS {
            l: [f32x8::splat(0.0); 64],
            r: [f32x8::splat(0.0); 64],
        }
    }

    pub fn zero(&mut self) {
        *self = ChunkS::new()
    }

    pub fn render(&self, dest: &mut ChunkS) {
        for i in 0..self.l.len() {
            dest.l[i] += self.l[i];
            dest.r[i] += self.r[i];
        }
    }
}

#[derive(Debug)]
pub struct ChunkM { 
    // 512 samples: 44100 samples per second; can track changes at 86hz
    pub c: [f32x8; 64],
}

impl ChunkM {
    pub fn new() -> ChunkM {
        ChunkM {
            c: [f32x8::splat(0.0); 64],
        }
    }

    pub fn zero(&mut self) {
        *self = ChunkM::new()
    }

    pub fn broadcast(&self) -> ChunkS {
        return ChunkS {
            l: self.c,
            r: self.c
        };
    }

    pub fn render(&self, dest: &mut ChunkM) {
        for i in 0..self.c.len() {
            dest.c[i] += self.c[i];
        }
    }
}