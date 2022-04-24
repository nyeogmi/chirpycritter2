use std::{collections::HashMap, sync::{Mutex, Arc}};

use super::{control::KeyOn, sound::Sound, Globals};

pub trait Player = 'static + Send + Fn(Globals, InstrumentView, KeyOn) -> Sound;

pub struct Instrument {
    rec: Arc<InstrumentRec>,
    play: Box<dyn Player>,
}

pub struct InstrumentController {
    rec: Arc<InstrumentRec>,
}


pub struct InstrumentView {
    rec: Arc<InstrumentRec>,
}

pub struct InstrumentBuilder {
    rec: InstrumentRec
}

struct InstrumentRec {
    param_names: HashMap<String, usize>,
    param_values: Mutex<Vec<f32>>,
}

#[derive(Clone, Copy)]
pub struct Param(usize);

impl Instrument {
    pub fn new<F: Player>(build: impl FnOnce(&mut InstrumentBuilder) -> F) -> Instrument {
        let mut builder = InstrumentBuilder {
            rec: InstrumentRec {
                param_names: HashMap::new(),
                param_values: Mutex::new(vec![]),
            }
        };

        let play = build(&mut builder);

        return Instrument { rec: Arc::new(builder.rec), play: Box::new(play) }
    }

    pub(crate) fn play(&self, globals: Globals, key: KeyOn) -> Sound {
        (self.play)(globals, InstrumentView::new(self.rec.clone()), key)
    }

    pub(crate) fn control(&self) -> InstrumentController {
        InstrumentController { rec: self.rec.clone() }
    }
}

impl InstrumentController {
    pub fn get_param(&self, name: &str) -> Option<f32> {
        if let Some(ix) = self.rec.param_names.get(name) {
            let mv = self.rec.param_values.lock().unwrap();
            return Some(mv[*ix])
        }
        return None
    }

    pub fn set_param(&self, name: &str, value: f32) {
        if let Some(ix) = self.rec.param_names.get(name) {
            let mut mv = self.rec.param_values.lock().unwrap();
            mv[*ix] = value;
        }
    }
}

impl InstrumentBuilder {
    pub fn param(&mut self, name: String, default_value: f32) -> Param {
        if let Some(ix) = self.rec.param_names.get(&name) {
            let mv = self.rec.param_values.get_mut().unwrap();
            mv[*ix] = default_value;
            return Param(*ix)
        }

        let mv = self.rec.param_values.get_mut().unwrap();  // shouldn't crash: only the builder holds the vec
        let ix = mv.len();
        mv.push(default_value);
        self.rec.param_names.insert(name, ix);

        assert_eq!(self.rec.param_names.len(), mv.len());
        Param(ix)
    }
}
impl InstrumentView {
    fn new(rec: Arc<InstrumentRec>) -> InstrumentView {
        InstrumentView { rec }
    }

    pub fn get_param(&self, param: Param) -> f32 {
        let mv = self.rec.param_values.lock().unwrap();
        mv[param.0]
    }
}


