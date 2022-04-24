use super::{control::KeyState, chunk::ChunkS};

pub trait Renderer = 'static + Send + FnMut(KeyState, &mut ChunkS) -> bool;  // return false when the note is no longer playing

pub struct Sound {
    keystate: KeyState,
    render: Box<dyn Renderer>,
    done: bool,
}

// TODO: Track keystate
impl Sound {
    pub fn new<F: Renderer>(render: F) -> Sound {
        Sound {
            keystate: KeyState { pressed: true },
            render: Box::new(render),
            done: false,
        }
    }

    pub fn release(&mut self) {
        self.keystate.pressed = false;
    }

    pub fn render(&mut self, chunk: &mut ChunkS) {
        if !(self.render)(self.keystate, chunk) {
            self.done = true;
        }
    }

    pub fn is_done(&self) -> bool {
        return self.done;
    }
}