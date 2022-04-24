#![feature(array_zip)]
#![feature(hash_drain_filter)]
#![feature(trait_alias)]

mod engine;
mod host;

use core::time;
use std::thread;

use crate::engine::*;
use crate::host::*;

fn main() {
    let inst = Instrument::new(|blder: &mut InstrumentBuilder| {
        let pitch_bend = blder.param("pitch_bend".to_string(), 0.0);

        return move |globals: Globals, me: InstrumentView, key: KeyOn| {
            let note = key.note;
            let volume = key.velocity as f32 / 127.0;

            let mut generator = WavetableM::new(globals, Sine {}, 0.0, volume);
            let mut mono_chunk = ChunkM::new();

            Sound::new(move |key: KeyState, samples: &mut ChunkS| {
                let frequency2 = midi_note_to_frequency(note as f32 + me.get_param(pitch_bend));
                generator.frequency = frequency2;

                mono_chunk.zero();
                generator.render(&mut mono_chunk);
                mono_chunk.broadcast().render(samples);

                return key.pressed;
            })
        }
    });
    let inst_ctrl = inst.control();

    let realtime = Realtime::start();
    let ensemble = realtime.ensemble.clone();

    let channel = {
        let mut e = ensemble.lock().unwrap();
        let track = e.add_track();
        let channel = e.add_channel(inst, track);
        e.play(channel, KeyOn { note: 64, velocity: 80 });
        channel
    };

    let mut i = 0;
    while ensemble.lock().unwrap().is_playing() {
        thread::sleep(time::Duration::from_millis(10)); // TODO: More sensible policy for this
        i += 1;
        inst_ctrl.set_param("pitch_bend", i as f32 / 10.0);

        if i == 150 {
            ensemble.lock().unwrap().release(channel, KeyOff { note: 64 });
        }
    }
}

fn midi_note_to_frequency(note: f32) -> f32 {
    return 2.0_f32.powf((note - 69.0) / 12.0) * 440.0;
}
