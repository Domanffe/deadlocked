use utils::bitset::BitSet;

use crate::{
    cs2::{key_codes::KeyCode, offsets::Offsets},
    os::process::Process,
};

#[derive(Debug)]
pub struct Input {
    previous_state: BitSet<64>,
    current_state: BitSet<64>,
}

impl Input {
    const MAX_KEY: u64 = 512;

    pub fn new() -> Self {
        Self {
            previous_state: BitSet::new(),
            current_state: BitSet::new(),
        }
    }

    pub fn update(&mut self, process: &Process, offsets: &Offsets) {
        let state = process.read_vec(
            offsets.interface.input + offsets.direct.button_state,
            (Self::MAX_KEY / 8) as usize,
        );

        std::mem::swap(&mut self.previous_state, &mut self.current_state);
        self.current_state = match state.try_into() {
            Ok(bytes) => BitSet::from_bytes(bytes),
            Err(_) => BitSet::new(),
        };
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.current_state.get(key.usize()).unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn key_just_pressed(&self, key: KeyCode) -> bool {
        !self.previous_state.get(key.usize()).unwrap_or(false)
            && self.current_state.get(key.usize()).unwrap_or(false)
    }
}
