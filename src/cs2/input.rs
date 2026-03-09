use utils::bitset::BitSet;

use crate::{
    cs2::{key_codes::KeyCode, offsets::Offsets},
    os::process::Process,
};

#[derive(Debug)]
pub struct Input {
    previous_state: BitSet,
    current_state: BitSet,
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
        let state = process.read_bytes(
            offsets.interface.input + offsets.direct.button_state,
            Self::MAX_KEY / 8,
        );

        std::mem::swap(&mut self.previous_state, &mut self.current_state);
        self.current_state = BitSet::from_vec(state);
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.current_state.get(key.usize())
    }

    #[allow(dead_code)]
    pub fn key_just_pressed(&self, key: KeyCode) -> bool {
        !self.previous_state.get(key.usize()) && self.current_state.get(key.usize())
    }
}
