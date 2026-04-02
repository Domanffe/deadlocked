use crate::{
    os::process::Process,
    parser::bvh::{Bvh, Triangle},
};

pub fn load_runtime_map(process: &Process, vphys_world: u64) -> Option<Bvh> {
    if process.pid <= 0 || vphys_world == 0 {
        return None;
    }

    let triangles = unsafe { read_bvh(process.pid, vphys_world as usize) }?;
    if triangles.is_empty() {
        return None;
    }

    let mut bvh = Bvh::new();
    bvh.set(triangles);
    bvh.build();
    Some(bvh)
}

unsafe extern "Rust" {
    fn read_bvh(pid: i32, vphys_world: usize) -> Option<Vec<Triangle>>;
}
