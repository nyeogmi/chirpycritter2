#[derive(Clone, Copy)]
pub struct KeyOn {
    pub note: u8,
    pub velocity: u8,
}

#[derive(Clone, Copy)]
pub struct KeyOff {
    pub note: u8,
}

#[derive(Clone, Copy)]
pub struct KeyState {
    pub pressed: bool,
}