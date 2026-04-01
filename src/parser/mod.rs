use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Write as _},
    path::{Path, PathBuf},
    process::Command,
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicU8, AtomicUsize, Ordering},
    },
};

use bytemuck::AnyBitPattern;
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use utils::log;

use crate::{
    os::crash::{self},
    parser::bvh::{Bvh, Triangle},
};

pub mod bvh;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserState {
    Idle,
    Parsing,
    Ready,
    Failed,
}

#[derive(Debug, Clone)]
pub struct ParserStatus {
    pub state: ParserState,
    pub parsed_maps: usize,
    pub total_maps: usize,
    pub current_map: String,
}

static PARSER_STATE: AtomicU8 = AtomicU8::new(0);
static PARSED_MAPS: AtomicUsize = AtomicUsize::new(0);
static TOTAL_MAPS: AtomicUsize = AtomicUsize::new(0);
static CURRENT_MAP: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

fn set_parser_state(state: ParserState) {
    let v = match state {
        ParserState::Idle => 0,
        ParserState::Parsing => 1,
        ParserState::Ready => 2,
        ParserState::Failed => 3,
    };
    PARSER_STATE.store(v, Ordering::Relaxed);
}

fn set_current_map(name: &str) {
    if let Ok(mut current) = CURRENT_MAP.lock() {
        *current = name.to_string();
    }
}

pub fn parser_status() -> ParserStatus {
    let state = match PARSER_STATE.load(Ordering::Relaxed) {
        1 => ParserState::Parsing,
        2 => ParserState::Ready,
        3 => ParserState::Failed,
        _ => ParserState::Idle,
    };
    let current_map = CURRENT_MAP
        .lock()
        .map(|s| s.clone())
        .unwrap_or_else(|_| String::new());
    ParserStatus {
        state,
        parsed_maps: PARSED_MAPS.load(Ordering::Relaxed),
        total_maps: TOTAL_MAPS.load(Ordering::Relaxed),
        current_map,
    }
}

pub fn parse_maps(mut force_reparse: bool, use_system_binary: bool) {
    set_parser_state(ParserState::Parsing);
    PARSED_MAPS.store(0, Ordering::Relaxed);
    TOTAL_MAPS.store(0, Ordering::Relaxed);
    set_current_map("");

    crash::info();
    let source2viewer = exe_path().join("source2viewer/Source2Viewer-CLI");
    if !source2viewer.exists() && !use_system_binary {
        log::warn!("could not find source2viewer binary");
        set_parser_state(ParserState::Failed);
        return;
    }

    let game_dir = match game_dir() {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("could not find cs2 game directory: {err}");
            set_parser_state(ParserState::Failed);
            return;
        }
    };
    let build_file = game_dir.join("game/bin/built_from_cl.txt");
    let Ok(cs2_build_raw) = std::fs::read_to_string(&build_file) else {
        log::warn!("could not read cs2 build number");
        set_parser_state(ParserState::Failed);
        return;
    };
    let cs2_build = cs2_build_raw.trim();
    let cs2_build: u64 = cs2_build.parse().unwrap_or_default();

    let maps_dir = match maps_dir() {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("could not find cs2 maps directory: {err}");
            set_parser_state(ParserState::Failed);
            return;
        }
    };
    let parsed_build_file = maps_dir.join("parsed_build.txt");
    let parsed_build = std::fs::read_to_string(&parsed_build_file).unwrap_or_default();
    let parsed_build: u64 = parsed_build.parse().unwrap_or_default();

    if parsed_build != cs2_build {
        force_reparse = true;
    }

    if force_reparse {
        log::info!("reparsing map data");
    }

    let mut files = Vec::with_capacity(32);
    let maps_dir_iter = match std::fs::read_dir(&maps_dir) {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("could not read cs2 maps dir: {err}");
            set_parser_state(ParserState::Failed);
            return;
        }
    };
    for file in maps_dir_iter {
        let Ok(file) = file else {
            continue;
        };

        let Ok(file_type) = file.file_type() else {
            continue;
        };
        if !file_type.is_file() {
            continue;
        }

        let file_name = file.file_name();
        let Some(file_name) = file_name.to_str() else {
            continue;
        };
        if file_name.contains("_vanity") {
            continue;
        }

        if !file_name.starts_with("ar_")
            && !file_name.starts_with("cs_")
            && !file_name.starts_with("de_")
        {
            continue;
        }

        if !file_name.ends_with(".vpk") {
            continue;
        }

        files.push(file_name.to_string());
    }

    let geom_dir = maps_dir.join("geometry");
    if force_reparse
        && geom_dir.exists()
        && let Err(err) = std::fs::remove_dir_all(&geom_dir)
    {
        log::error!("error removing geometry dir: {err}");
    }

    if !geom_dir.exists()
        && let Err(err) = std::fs::create_dir_all(geom_dir.join("maps"))
    {
        log::error!("error creating geometry dir: {err}");
    }
    for file in &files {
        let path = maps_dir.join(file);
        let map_name = file.trim_end_matches(".vpk");

        if maps_dir.join("geometry/maps").join(map_name).exists() && !force_reparse {
            continue;
        }

        let mut s2v_cmd = Command::new(if use_system_binary {
            std::ffi::OsStr::new("Source2Viewer-CLI")
        } else {
            source2viewer.as_os_str()
        });
        s2v_cmd.args([
            "-i",
            path.to_string_lossy().as_ref(),
            "-d",
            "-o",
            geom_dir.to_string_lossy().as_ref(),
            "-f",
            &format!("maps/{map_name}/world_physics.vmdl_c"),
        ]);
        if let Err(error) = s2v_cmd.output() {
            log::error!("source2viewer error:\n{error}");
        }
    }

    if !geom_dir.exists() {
        log::warn!("could not parse any map successfully");
        set_parser_state(ParserState::Failed);
        return;
    }

    let pending_maps: Vec<String> = files
        .iter()
        .filter(|file| {
            let map_name = file.trim_end_matches(".vpk");
            force_reparse || !maps_dir.join(format!("{map_name}.bvh")).exists()
        })
        .cloned()
        .collect();

    TOTAL_MAPS.store(pending_maps.len(), Ordering::Relaxed);
    if pending_maps.is_empty() {
        set_parser_state(ParserState::Ready);
        return;
    }

    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let batch_size = (cpus / 2).max(1);
    for chunk in pending_maps.chunks(batch_size) {
        let mut threads = Vec::with_capacity(batch_size);
        for map in chunk {
            let map = map.clone();
            let maps_dir = maps_dir.clone();
            let thread = std::thread::spawn(move || {
                set_current_map(&map);
                parse_map(&map, &maps_dir, force_reparse);
                PARSED_MAPS.fetch_add(1, Ordering::Relaxed);
            });
            threads.push(thread);
        }

        for thread in threads {
            let _ = thread.join();
        }
    }
    let mut parsed_build_file = match File::create(&parsed_build_file) {
        Ok(file) => file,
        Err(err) => {
            log::error!("could not open metadata file: {err}");
            set_parser_state(ParserState::Failed);
            return;
        }
    };
    if let Err(err) = parsed_build_file.write_all(format!("{cs2_build}").as_bytes()) {
        log::error!("could not write to metadata file: {err}");
        set_parser_state(ParserState::Failed);
        return;
    }
    set_current_map("");
    set_parser_state(ParserState::Ready);
    log::info!("loaded map info");
}

fn parse_map(map: &str, maps_dir: &Path, force_reparse: bool) {
    let map_name = map.replace(".vpk", "");
    let bvh_name = format!("{map_name}.bvh");
    let bvh_path = maps_dir.join(bvh_name);

    if bvh_path.exists() && !force_reparse {
        log::debug!("bvh for {map_name} exists");
        return;
    }

    let geom_dir = maps_dir.join("geometry/maps").join(&map_name);
    if !geom_dir.exists() {
        log::warn!("geometry directory doesn't exist...");
    }

    let mut map_bvh = Bvh::new();
    let geom_dir_iter = match std::fs::read_dir(&geom_dir) {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("could not read geometry directory: {err}");
            return;
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
                return;
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
            } else {
                for i in 1..face.len() - 1 {
                    let v1 = vertices[face[0] as usize];
                    let v2 = vertices[face[i] as usize];
                    let v3 = vertices[face[i + 1] as usize];
                    let triangle = Triangle::new(v1, v2, v3);
                    map_bvh.insert(triangle);
                }
            }
        }
    }
    map_bvh.build();
    let mut bvh_file = match File::create(&bvh_path) {
        Ok(file) => file,
        Err(err) => {
            log::error!("could not save bvh for {map_name} in file {bvh_path:?}: {err}");
            return;
        }
    };
    map_bvh.save(&mut bvh_file);
    log::info!("parsed bvh for {map_name}");
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

pub fn load_map(map_name: &str) -> Option<Bvh> {
    let maps_dir = maps_dir().ok()?;
    let bvh_name = if map_name.ends_with(".vpk") {
        map_name.replace(".vpk", ".bvh")
    } else {
        let mut name = map_name.to_owned();
        name.push_str(".bvh");
        name
    };
    let bvh_path = maps_dir.join(bvh_name);
    if !bvh_path.exists() {
        return None;
    }
    let mut bvh_file = File::open(&bvh_path).ok()?;
    Bvh::load(&mut bvh_file)
}
