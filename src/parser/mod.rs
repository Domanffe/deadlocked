use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    process::Command,
    sync::{LazyLock, Mutex},
};

use bytemuck::AnyBitPattern;
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use utils::log;

use crate::{
    os::process::Process,
    parser::bvh::{Bvh, Triangle},
};

pub mod bvh;
mod mapdata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometrySource {
    RuntimeMapdata,
    CacheBvh,
    Source2ViewerCurrentMap,
}

impl GeometrySource {
    pub fn label(self) -> &'static str {
        match self {
            Self::RuntimeMapdata => "Runtime",
            Self::CacheBvh => "Cache fallback",
            Self::Source2ViewerCurrentMap => "Current-map extract",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryState {
    Idle,
    Loading,
    Ready,
    Fallback,
    Failed,
}

impl GeometryState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Loading => "Loading",
            Self::Ready => "Ready",
            Self::Fallback => "Fallback",
            Self::Failed => "Failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeometryStatus {
    pub state: GeometryState,
    pub source: Option<GeometrySource>,
    pub map: String,
    pub detail: String,
}

impl Default for GeometryStatus {
    fn default() -> Self {
        Self {
            state: GeometryState::Idle,
            source: None,
            map: String::new(),
            detail: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct GeometryOptions {
    force_reparse: bool,
    use_system_binary: bool,
}

static GEOMETRY_STATUS: LazyLock<Mutex<GeometryStatus>> =
    LazyLock::new(|| Mutex::new(GeometryStatus::default()));
static GEOMETRY_OPTIONS: LazyLock<Mutex<GeometryOptions>> =
    LazyLock::new(|| Mutex::new(GeometryOptions::default()));

fn set_geometry_status(status: GeometryStatus) {
    if let Ok(mut current) = GEOMETRY_STATUS.lock() {
        *current = status;
    }
}

pub fn set_geometry_options(force_reparse: bool, use_system_binary: bool) {
    if let Ok(mut options) = GEOMETRY_OPTIONS.lock() {
        *options = GeometryOptions {
            force_reparse,
            use_system_binary,
        };
    }
}

fn geometry_options() -> GeometryOptions {
    GEOMETRY_OPTIONS
        .lock()
        .map(|options| *options)
        .unwrap_or_default()
}

pub fn geometry_status() -> GeometryStatus {
    GEOMETRY_STATUS
        .lock()
        .map(|status| status.clone())
        .unwrap_or_default()
}

pub fn load_map_auto(
    process: &Process,
    map_name: &str,
    vphys_world: u64,
) -> Option<(Bvh, GeometrySource)> {
    let map_name = map_name.trim_end_matches(".vpk").to_string();
    if map_name.is_empty() {
        set_geometry_status(GeometryStatus::default());
        return None;
    }

    set_geometry_status(GeometryStatus {
        state: GeometryState::Loading,
        source: None,
        map: map_name.clone(),
        detail: "Resolving geometry backend".to_string(),
    });

    let options = geometry_options();
    let (result, status) = resolve_geometry_load(
        &map_name,
        || mapdata::load_runtime_map(process, vphys_world),
        || load_cached_map(&map_name),
        || load_current_map_with_source2viewer(&map_name, options.force_reparse, options.use_system_binary),
    );

    if let Some((bvh, GeometrySource::RuntimeMapdata)) = result.as_ref() {
        warm_cache_async(map_name.clone(), bvh.clone());
    }

    set_geometry_status(status);
    result
}

fn resolve_geometry_load<RuntimeLoad, CacheLoad, RecoveryLoad>(
    map_name: &str,
    runtime_load: RuntimeLoad,
    cache_load: CacheLoad,
    recovery_load: RecoveryLoad,
) -> (Option<(Bvh, GeometrySource)>, GeometryStatus)
where
    RuntimeLoad: FnOnce() -> Option<Bvh>,
    CacheLoad: FnOnce() -> Option<Bvh>,
    RecoveryLoad: FnOnce() -> Option<Bvh>,
{
    if let Some(bvh) = runtime_load() {
        return (
            Some((bvh, GeometrySource::RuntimeMapdata)),
            GeometryStatus {
                state: GeometryState::Ready,
                source: Some(GeometrySource::RuntimeMapdata),
                map: map_name.to_string(),
                detail: "Loaded live geometry from game memory".to_string(),
            },
        );
    }

    if let Some(bvh) = cache_load() {
        return (
            Some((bvh, GeometrySource::CacheBvh)),
            GeometryStatus {
                state: GeometryState::Fallback,
                source: Some(GeometrySource::CacheBvh),
                map: map_name.to_string(),
                detail: "Runtime backend unavailable, using cached .bvh".to_string(),
            },
        );
    }

    if let Some(bvh) = recovery_load() {
        return (
            Some((bvh, GeometrySource::Source2ViewerCurrentMap)),
            GeometryStatus {
                state: GeometryState::Fallback,
                source: Some(GeometrySource::Source2ViewerCurrentMap),
                map: map_name.to_string(),
                detail: "Recovered current map via Source2Viewer".to_string(),
            },
        );
    }

    (
        None,
        GeometryStatus {
            state: GeometryState::Failed,
            source: None,
            map: map_name.to_string(),
            detail: "No runtime geometry, cache, or recovery path succeeded".to_string(),
        },
    )
}

fn warm_cache_async(map_name: String, bvh: Bvh) {
    std::thread::spawn(move || {
        if save_cached_map(&map_name, &bvh).is_none() {
            log::warn!("failed to warm geometry cache for {map_name}");
        }
    });
}

fn load_current_map_with_source2viewer(
    map_name: &str,
    force_reparse: bool,
    use_system_binary: bool,
) -> Option<Bvh> {
    let maps_dir = maps_dir().ok()?;
    let map_path = maps_dir.join(vpk_name(map_name));
    if !map_path.exists() {
        log::warn!("map file is missing for Source2Viewer recovery: {}", map_path.display());
        return None;
    }

    let source2viewer = resolve_source2viewer_binary(use_system_binary)?;
    let geom_dir = maps_dir.join("geometry");
    let map_geom_dir = geom_dir.join("maps").join(map_name);

    if force_reparse && map_geom_dir.exists() && std::fs::remove_dir_all(&map_geom_dir).is_err() {
        log::warn!("failed to clear old geometry for {map_name}");
    }
    if std::fs::create_dir_all(geom_dir.join("maps")).is_err() {
        log::warn!("failed to prepare geometry output directory");
        return None;
    }

    log::info!("recovering geometry for {map_name} via Source2Viewer");
    let output = match Command::new(&source2viewer)
        .args([
            "-i",
            map_path.to_string_lossy().as_ref(),
            "-d",
            "-o",
            geom_dir.to_string_lossy().as_ref(),
            "-f",
            &format!("maps/{map_name}/world_physics.vmdl_c"),
        ])
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            log::warn!("failed to start Source2Viewer for {map_name}: {err}");
            return None;
        }
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::warn!("Source2Viewer failed for {map_name}: {}", stderr.trim());
        return None;
    }

    parse_map(map_name, &maps_dir, force_reparse)
}

fn resolve_source2viewer_binary(use_system_binary: bool) -> Option<PathBuf> {
    if use_system_binary {
        return Some(PathBuf::from("Source2Viewer-CLI"));
    }

    let source2viewer = exe_path().join("source2viewer/Source2Viewer-CLI");
    if source2viewer.exists() {
        Some(source2viewer)
    } else {
        None
    }
}

fn parse_map(map_name: &str, maps_dir: &Path, force_reparse: bool) -> Option<Bvh> {
    let bvh_path = maps_dir.join(bvh_name(map_name));

    if bvh_path.exists() && !force_reparse {
        return load_bvh_from_path(&bvh_path);
    }

    let geom_dir = maps_dir.join("geometry/maps").join(map_name);
    if !geom_dir.exists() {
        log::warn!("geometry directory is missing for {map_name}: {}", geom_dir.display());
        return None;
    }

    let mut map_bvh = Bvh::new();
    let mut found_geometry = false;
    let geom_dir_iter = match std::fs::read_dir(&geom_dir) {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("could not read geometry directory: {err}");
            return None;
        }
    };
    for file in geom_dir_iter {
        let Ok(file) = file else {
            continue;
        };
        let file_name = file.file_name();
        let Some(file_name) = file_name.to_str() else {
            continue;
        };
        let file_type = if file_name.contains("world_physics_hull") {
            FileType::Hull
        } else if file_name.contains("world_physics_phys") {
            FileType::Phys
        } else {
            continue;
        };
        let file = match File::open(file.path()) {
            Ok(file) => file,
            Err(err) => {
                log::error!("could not open {file_name} ({map_name}): {err}");
                return None;
            }
        };
        let mut reader = BufReader::new(file);
        let elements = parse_dmx(&mut reader);

        // DmeMaterial_material.mtlName == "flags$kind"
        let Some(material_element) = elements.get("DmeMaterial_material") else {
            continue;
        };
        let Some(Attribute::String(material)) = material_element.attributes.get("mtlName") else {
            continue;
        };
        if !material.starts_with('$') {
            continue;
        }

        let Some(vertex_element) = elements.get("DmeVertexData_bind") else {
            continue;
        };
        let Some(Attribute::Vec3Array(vertices)) = vertex_element.attributes.get("position$0")
        else {
            continue;
        };
        let vertex_indices: Vec<&[i32]> = if file_type == FileType::Hull {
            let Some(face_element) = elements.get("DmeFaceSet_hull faces") else {
                continue;
            };
            let Some(Attribute::IntegerArray(indices)) = face_element.attributes.get("faces")
            else {
                continue;
            };
            indices.split(|i| *i == -1).collect()
        } else {
            let Some(Attribute::IntegerArray(indices)) =
                vertex_element.attributes.get("position$0Indices")
            else {
                continue;
            };
            indices.chunks_exact(3).collect()
        };

        for face in vertex_indices {
            if face.len() < 3 || face.iter().any(|index| *index as usize >= vertices.len()) {
                continue;
            } else if face.len() == 3 {
                let v1 = vertices[face[0] as usize];
                let v2 = vertices[face[1] as usize];
                let v3 = vertices[face[2] as usize];
                let triangle = Triangle::new(v1, v2, v3);
                map_bvh.insert(triangle);
                found_geometry = true;
            } else {
                for i in 1..face.len() - 1 {
                    let v1 = vertices[face[0] as usize];
                    let v2 = vertices[face[i] as usize];
                    let v3 = vertices[face[i + 1] as usize];
                    let triangle = Triangle::new(v1, v2, v3);
                    map_bvh.insert(triangle);
                    found_geometry = true;
                }
            }
        }
    }
    if !found_geometry {
        log::warn!("no geometry triangles were extracted for {map_name}");
        return None;
    }
    map_bvh.build();
    let _ = save_bvh_to_path(&bvh_path, &map_bvh);
    log::info!("parsed bvh for {map_name}");
    Some(map_bvh)
}

#[derive(PartialEq)]
enum FileType {
    Hull,
    Phys,
}

fn read_element(reader: &mut impl Read, strings: &[String]) -> Element {
    let kind = &strings[read::<i32>(reader) as usize];
    let name = &strings[read::<i32>(reader) as usize];
    let _uuid = read_bytes(reader, 16);
    Element::new(kind.to_string(), name.to_string())
}

fn parse_dmx(reader: &mut impl Read) -> HashMap<String, Element> {
    let _header = read_string(reader);
    let _prefix_elements: i32 = read(reader);
    let string_count: i32 = read(reader);
    let mut strings = Vec::with_capacity(string_count as usize);
    for _ in 0..string_count {
        strings.push(read_string(reader));
    }

    let element_count: i32 = read(reader);
    let mut elements = Vec::with_capacity(element_count as usize);
    for _ in 0..element_count {
        let element = read_element(reader, &strings);
        elements.push(element);
    }

    for element in &mut elements {
        let attribute_count: i32 = read(reader);
        for _ in 0..attribute_count {
            let name = &strings[read::<i32>(reader) as usize];
            let kind: u8 = read(reader);
            use Attribute as AT;
            let value = match kind {
                1 => AT::Element({
                    let index: i32 = read(reader);
                    if index < 0 { None } else { Some(index) }
                }),
                2 => AT::Integer(read(reader)),
                3 => AT::Float(read(reader)),
                4 => AT::Bool(read::<u8>(reader) != 0),
                5 => AT::String(strings[read::<i32>(reader) as usize].clone()),
                6 => AT::ByteArray({
                    let count: i32 = read(reader);
                    read_bytes(reader, count as usize)
                }),
                7 => AT::TimeSpan(read(reader)),
                8 => AT::Color(read(reader)),
                9 => AT::Vec2(read(reader)),
                10 => AT::Vec3(read(reader)),
                11 => AT::Angle(read(reader)),
                12 => AT::Vec4(read(reader)),
                13 => AT::Quaternion(read(reader)),
                14 => AT::Matrix(read(reader)),
                15 => AT::Byte(read(reader)),
                16 => AT::U64(read(reader)),

                33 => AT::ElementArray({
                    let count: i32 = read(reader);
                    (0..count)
                        .map(|_| {
                            let idx: i32 = read(reader);
                            match idx {
                                -1 => None,
                                -2 => None,
                                x => Some(x),
                            }
                        })
                        .collect()
                }),
                34 => AT::IntegerArray({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),
                35 => AT::FloatArray({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),
                36 => AT::BoolArray({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read::<u8>(reader) != 0).collect()
                }),
                37 => AT::StringArray({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read_string(reader)).collect()
                }),
                38 => AT::ByteArray(Vec::new()),
                39 => AT::TimeSpanArray({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),
                40 => AT::ColorArray({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),
                41 => AT::Vec2Array({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),
                42 => AT::Vec3Array({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),
                43 => AT::AngleArray({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),
                44 => AT::Vec4Array({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),

                45 => AT::QuaternionArray({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),
                46 => AT::MatrixArray({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),
                47 => AT::ByteArray({
                    let count: i32 = read(reader);
                    read_bytes(reader, count as usize)
                }),
                48 => AT::U64Array({
                    let count: i32 = read(reader);
                    (0..count).map(|_| read(reader)).collect()
                }),

                _ => AT::ByteArray(Vec::new()),
            };
            element.add(name.to_string(), value);
        }
    }
    let mut elems = HashMap::new();
    elements.into_iter().for_each(|e| {
        let name = format!("{}_{}", e.kind, e.name);
        elems.insert(name, e);
    });

    elems
}

#[derive(Debug, Clone)]
struct Element {
    kind: String,
    name: String,
    attributes: HashMap<String, Attribute>,
}

impl Element {
    pub fn new(kind: String, name: String) -> Self {
        Self {
            kind,
            name,
            attributes: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, attribute: Attribute) {
        self.attributes.insert(name, attribute);
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
enum Attribute {
    // element index
    Element(Option<i32>),
    Integer(i32),
    Float(f32),
    Bool(bool),
    String(String),
    ByteArray(Vec<u8>),
    TimeSpan(i32),
    Color(u32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Angle(Vec3),
    Quaternion(Quat),
    Matrix(Mat4),
    U64(u64),
    Byte(u8),

    ElementArray(Vec<Option<i32>>),
    IntegerArray(Vec<i32>),
    FloatArray(Vec<f32>),
    BoolArray(Vec<bool>),
    StringArray(Vec<String>),
    TimeSpanArray(Vec<i32>),
    ColorArray(Vec<u32>),
    Vec2Array(Vec<Vec2>),
    Vec3Array(Vec<Vec3>),
    Vec4Array(Vec<Vec4>),
    AngleArray(Vec<Vec3>),
    QuaternionArray(Vec<Quat>),
    MatrixArray(Vec<Mat4>),
    U64Array(Vec<u64>),
}

// todo: improve this
fn game_dir() -> Result<PathBuf, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let sudo_user = std::env::var("SUDO_USER").ok();

    let mut steam_path = PathBuf::from(&home).join(".steam/steam");
    if !steam_path.exists()
        && let Some(user) = sudo_user
    {
        let user_home = PathBuf::from("/home").join(user);
        steam_path = user_home.join(".steam/steam");
    }

    if !steam_path.exists() {
        log::error!("please install steam native, not from flatpak.");
        return Err(format!(
            "could not locate steam directory. checked: {}/.steam/steam",
            home
        ));
    }

    let library_folders = steam_path.join("config/libraryfolders.vdf");
    let Ok(content) = std::fs::read_to_string(&library_folders) else {
        return Err(format!(
            "could not read steam library folders ({})",
            library_folders.display()
        ));
    };
    let libs: Vec<&str> = content
        .lines()
        .filter_map(|line| {
            if line.contains("\"path\"") {
                line.rsplit('"').nth(1)
            } else {
                None
            }
        })
        .collect();

    let game_dir = libs
        .iter()
        .find(|&&lib| {
            let dir = PathBuf::from(lib).join("steamapps/common/Counter-Strike Global Offensive");
            dir.exists()
        })
        .ok_or("could not locate cs2 files. is it installed?".to_owned())?;
    Ok(PathBuf::from(game_dir).join("steamapps/common/Counter-Strike Global Offensive"))
}

fn maps_dir() -> Result<PathBuf, String> {
    let maps_dir = game_dir().map(|p| p.join("game/csgo/maps"))?;
    if !maps_dir.exists() {
        Err("could locate csgo directory, but not maps directory".to_string())
    } else {
        Ok(maps_dir)
    }
}

fn exe_path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn read<T: AnyBitPattern + Default>(reader: &mut impl Read) -> T {
    let mut buffer = vec![0u8; size_of::<T>()];
    if reader.read_exact(&mut buffer).is_err() {
        return T::default();
    }
    *bytemuck::from_bytes(&buffer)
}

fn read_string(reader: &mut impl Read) -> String {
    let mut buffer = Vec::with_capacity(8);
    let mut byte = [0u8; 1];

    loop {
        if reader.read_exact(&mut byte).is_err() {
            break;
        }
        if byte[0] == 0 {
            break;
        }
        buffer.push(byte[0]);
    }
    String::from_utf8(buffer).unwrap_or_default()
}

fn read_bytes(reader: &mut impl Read, count: usize) -> Vec<u8> {
    let mut buf = vec![0u8; count];
    let _ = reader.read_exact(&mut buf);
    buf
}

fn vpk_name(map_name: &str) -> PathBuf {
    let mut path = PathBuf::from(map_name.trim_end_matches(".vpk"));
    path.set_extension("vpk");
    path
}

fn bvh_name(map_name: &str) -> PathBuf {
    let mut path = PathBuf::from(map_name.trim_end_matches(".vpk"));
    path.set_extension("bvh");
    path
}

fn cache_path(map_name: &str) -> Option<PathBuf> {
    Some(maps_dir().ok()?.join(bvh_name(map_name)))
}

fn load_cached_map(map_name: &str) -> Option<Bvh> {
    let bvh_path = cache_path(map_name)?;
    load_bvh_from_path(&bvh_path)
}

fn load_bvh_from_path(path: &Path) -> Option<Bvh> {
    if !path.exists() {
        return None;
    }
    let mut bvh_file = File::open(path).ok()?;
    Bvh::load(&mut bvh_file)
}

fn save_cached_map(map_name: &str, bvh: &Bvh) -> Option<()> {
    let bvh_path = cache_path(map_name)?;
    save_bvh_to_path(&bvh_path, bvh)
}

fn save_bvh_to_path(path: &Path, bvh: &Bvh) -> Option<()> {
    if let Some(parent) = path.parent()
        && std::fs::create_dir_all(parent).is_err()
    {
        log::warn!("failed to create cache directory {}", parent.display());
        return None;
    }

    let mut bvh_file = File::create(path).ok()?;
    bvh.save(&mut bvh_file);
    Some(())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::{Bvh, GeometrySource, GeometryState, resolve_geometry_load};

    #[test]
    fn runtime_source_wins_before_fallbacks() {
        let calls = RefCell::new(Vec::new());
        let (_result, status) = resolve_geometry_load(
            "de_test",
            || {
                calls.borrow_mut().push("runtime");
                Some(Bvh::new())
            },
            || {
                calls.borrow_mut().push("cache");
                Some(Bvh::new())
            },
            || {
                calls.borrow_mut().push("s2v");
                Some(Bvh::new())
            },
        );

        assert_eq!(calls.into_inner(), vec!["runtime"]);
        assert_eq!(status.state, GeometryState::Ready);
        assert_eq!(status.source, Some(GeometrySource::RuntimeMapdata));
    }

    #[test]
    fn cache_fallback_is_used_before_source2viewer() {
        let calls = RefCell::new(Vec::new());
        let (_result, status) = resolve_geometry_load(
            "de_test",
            || {
                calls.borrow_mut().push("runtime");
                None
            },
            || {
                calls.borrow_mut().push("cache");
                Some(Bvh::new())
            },
            || {
                calls.borrow_mut().push("s2v");
                Some(Bvh::new())
            },
        );

        assert_eq!(calls.into_inner(), vec!["runtime", "cache"]);
        assert_eq!(status.state, GeometryState::Fallback);
        assert_eq!(status.source, Some(GeometrySource::CacheBvh));
    }

    #[test]
    fn source2viewer_recovery_runs_after_runtime_and_cache_fail() {
        let calls = RefCell::new(Vec::new());
        let (_result, status) = resolve_geometry_load(
            "de_test",
            || {
                calls.borrow_mut().push("runtime");
                None
            },
            || {
                calls.borrow_mut().push("cache");
                None
            },
            || {
                calls.borrow_mut().push("s2v");
                Some(Bvh::new())
            },
        );

        assert_eq!(calls.into_inner(), vec!["runtime", "cache", "s2v"]);
        assert_eq!(status.state, GeometryState::Fallback);
        assert_eq!(status.source, Some(GeometrySource::Source2ViewerCurrentMap));
    }

    #[test]
    fn failed_status_is_reported_when_all_backends_fail() {
        let (_result, status) =
            resolve_geometry_load("de_test", || None, || None, || None);

        assert_eq!(status.state, GeometryState::Failed);
        assert_eq!(status.source, None);
    }
}
