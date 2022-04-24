use std::{cell::RefCell, sync::{Arc, Mutex}, collections::VecDeque, borrow::Borrow};

use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}, Stream};

use crate::engine::{Ensemble, ChunkS, Globals};

pub struct Realtime {
    pub ensemble: Arc<Mutex<Ensemble>>,

    #[allow(dead_code)]
    stream: Stream,  // keep this so it doesn't get freed
}

impl Realtime {
    pub fn start() -> Realtime {
        let host = cpal::default_host();

        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();

        let sample_format = config.sample_format();
        let stream_config: cpal::StreamConfig = config.into();

        let sample_rate = stream_config.sample_rate.0 as u64;
        let channels = stream_config.channels as usize;

        // TODO
        assert_eq!(channels, 2);

        let ensemble = Arc::new(Mutex::new(Ensemble::new(Globals { sample_rate })));

        let ensemble2 = ensemble.clone();
        let stream = match (sample_format, channels) {
            (cpal::SampleFormat::F32, 2) => Self::run_stereo::<f32>(ensemble2, &device, &stream_config),
            (cpal::SampleFormat::I16, 2) => Self::run_stereo::<i16>(ensemble2, &device, &stream_config),
            (cpal::SampleFormat::U16, 2) => Self::run_stereo::<u16>(ensemble2, &device, &stream_config),
            _ => panic!("don't know how to run in mono yet"),
        };

        Realtime { 
            ensemble,
            stream,
        }
    }

    fn run_stereo<T>(ensemble: Arc<Mutex<Ensemble>>, device: &cpal::Device, config: &cpal::StreamConfig) -> Stream
        where T: cpal::Sample
    {
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let mut hot = HotStream::new();

        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                hot.pump(data, |x| ensemble.lock().unwrap().render(x));
            },
            err_fn,
        ).unwrap();
        stream.play().unwrap();
        stream
    }
}

struct HotStream {
    buffer: ChunkS,
    frames: VecDeque<f32>,
}

impl HotStream {
    pub fn new() -> Self {
        HotStream {
            buffer: ChunkS::new(),
            frames: VecDeque::new(),
        }
    }

    pub fn pump<T>(&mut self, output: &mut [T], get_more: impl Fn(&mut ChunkS)) where T: cpal::Sample {
        let mut output_i = 0;
        loop {
            let frames_needed = output.len() - output_i;
            if frames_needed <= 0 { break; }

            assert!(frames_needed % 2 == 0);

            while self.frames.len() < frames_needed {
                self.buffer.zero();
                get_more(&mut self.buffer);
                for i in 0..self.buffer.l.len() {
                    for i in self.buffer.l[i].to_array().zip(self.buffer.r[i].to_array()) {
                        self.frames.push_back(i.0);
                        self.frames.push_back(i.1);
                    }
                }
            }

            while output_i < output.len() {
                output[output_i] = cpal::Sample::from::<f32>(&self.frames.pop_front().unwrap());
                output_i += 1
            }
        }
    }
}