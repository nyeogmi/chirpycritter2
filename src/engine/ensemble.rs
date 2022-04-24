use std::{collections::{HashMap, HashSet, VecDeque}, cell::RefCell, borrow::BorrowMut};

use super::{instrument::Instrument, chunk::ChunkS, sound::Sound, KeyOn, Globals, KeyOff};

pub struct Ensemble {
    globals: Globals,

    pub channels: Vec<InternalChannel>,
    pub tracks: Vec<InternalTrack>,

    render_order: Vec<usize>,
    sink: ChunkS,
    chunks: Vec<RefCell<ChunkS>>,  
}

pub struct InternalChannel {
    instrument: Instrument,
    track: usize,
    notes: HashMap<u8, Sound>,
}

#[derive(Clone, Copy)]
pub struct Channel(usize);

pub struct InternalTrack {
    volume: f32,
    pan: f32,
    dest: Option<usize>,
    // TODO: Effects
}

#[derive(Clone, Copy)]
pub struct Track(usize);

impl Ensemble {
    pub fn new(globals: Globals) -> Ensemble {
        Ensemble {
            globals,
            channels: Vec::new(),
            tracks: Vec::new(),
            render_order: Vec::new(),
            sink: ChunkS::new(),
            chunks: Vec::new(),
        }
    }

    pub fn add_channel(&mut self, instrument: Instrument, track: Track) -> Channel {
        let ix = self.channels.len();
        self.channels.push(InternalChannel {
            instrument,
            track: track.0,
            notes: HashMap::new(),
        });
        return Channel(ix);
    }

    pub fn add_track(&mut self) -> Track {
        let ix = self.tracks.len();
        self.tracks.push(InternalTrack {
            volume: 1.0,
            pan: 0.0,
            dest: None,
        });
        return Track(ix);
    }

    pub fn play(&mut self, channel: Channel, key: KeyOn) {
        let ix = channel.0;
        let channel = self.channels.get_mut(ix).unwrap();
        let sound = channel.instrument.play(self.globals, key);
        channel.notes.insert(key.note, sound);
    }

    pub fn release(&mut self, channel: Channel, key: KeyOff) {
        let ix = channel.0;
        let channel = self.channels.get_mut(ix).unwrap();
        if let Some(sound) = channel.notes.get_mut(&key.note) {
            sound.release()
        }
    }

    pub fn render(&mut self, out: &mut ChunkS) {
        if self.render_order.len() == 0 && self.tracks.len() != 0 { self.compute_render_order() }

        while self.chunks.len() < self.tracks.len() { 
            self.chunks.push(RefCell::new(ChunkS::new()));
        }

        for i in &self.render_order {
            self.chunks[*i].borrow_mut().zero();
        }

        // seed each channel with its instrument data for its channels
        for (ix, channel) in self.channels.iter_mut().enumerate() {
            if channel.track > self.chunks.len() { channel.render(&mut self.sink) }
            else {
                channel.render(&mut self.chunks[channel.track].borrow_mut())
            }
        }

        for i in &self.render_order {
            if let Some(dest) = self.tracks[*i].dest {
                if *i == dest { continue } 
                self.chunks[*i].borrow_mut().render(&mut self.chunks[dest].borrow_mut())
            }
            else {
                self.chunks[*i].borrow_mut().render(out)
            }
        }
    }

    pub fn is_playing(&self) -> bool {
        // TODO: Also check if the currently playing score is finished, when that exists
        for channel in &self.channels {
            if !channel.notes.is_empty() { return true }
        }
        return false
    }

    fn compute_render_order(&mut self) {
        // Topologically sort tracks by `dest`, finishing with track 0
        let mut roots = VecDeque::new();
        let mut sources = Vec::new();
        for _ in 0..self.tracks.len() {
            sources.push(vec![]);
        }
        for i in 0..self.tracks.len() {
            if let Some(dest) = self.tracks[i].dest {
                sources[dest].push(i);
            } else {
                roots.push_back(i);  // goes to master
            }
        }

        let mut reached = HashSet::new();
        let mut to_visit = roots;
        let mut rev_order = Vec::new();

        // add everyone in dependency order
        while let Some(top) = to_visit.pop_front() {
            rev_order.push(top);
            reached.insert(top);
            for i in sources[top].iter() {
                if !reached.contains(i) { to_visit.push_back(*i); }
            }
        }

        rev_order.reverse();
        self.render_order = rev_order;
    }
}

impl InternalChannel {
    fn render(&mut self, chunk: &mut ChunkS) {
        // TODO: Remove sounds that are done playing
        for (ix, sound) in self.notes.iter_mut() {
            sound.render(chunk);
        }

        self.notes.drain_filter(|_, x| x.is_done());
    }
}