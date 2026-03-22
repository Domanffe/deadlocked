use std::{
    collections::HashMap,
    fs::read_to_string,
    ops::RangeInclusive,
    path::{Path, PathBuf},
    sync::LazyLock,
    time::Duration,
};

use egui::Color32;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use utils::log;

use crate::{
    cs2::{bones::Bones, entity::weapon::Weapon, key_codes::KeyCode},
    ui::color::Colors,
};

const REFRESH_RATE: u64 = 100;
pub const LOOP_DURATION: Duration = Duration::from_millis(1000 / REFRESH_RATE);
pub const SLEEP_DURATION: Duration = Duration::from_secs(5);
pub const DEFAULT_CONFIG_NAME: &str = "deadlocked.toml";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Language {
    English,
    Russian,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum SnaplineStart {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub language: Language,
    pub aim: AimConfig,
    pub player: PlayerConfig,
    pub hud: HudConfig,
    pub radar: RadarConfig,
    pub misc: UnsafeConfig,
    pub accent_color: Color32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: Language::English,
            aim: AimConfig::default(),
            player: PlayerConfig::default(),
            hud: HudConfig::default(),
            radar: RadarConfig::default(),
            misc: UnsafeConfig::default(),
            accent_color: Colors::BLUE,
        }
    }
}

impl Config {
    pub fn load_preset(&mut self, tier: u32) {
        let lang = self.language;
        let accent = self.accent_color;
        *self = Config::default();
        self.language = lang;
        self.accent_color = accent;

        match tier {
            0 => {
                // Legit
                self.aim.global.aimbot.fov = 1.2;
                self.aim.global.aimbot.smooth = 15.0;
                self.aim.global.aimbot.hitchance = 60.0;
                self.aim.global.aimbot.backtrack = false;
                self.aim.global.aimbot.multipoint = false;
                self.player.draw_skeleton = DrawMode::None;
                self.player.draw_box = DrawMode::None;
                self.player.box_thickness = 1.0;
                self.player.snaplines = false;
                self.hud.hitmarker = true;
            }
            1 => {
                // Semi-Legit
                self.aim.global.aimbot.fov = 2.0;
                self.aim.global.aimbot.smooth = 10.0;
                self.aim.global.aimbot.hitchance = 85.0;
                self.aim.global.aimbot.backtrack = true;
                self.aim.global.aimbot.backtrack_ticks = 6;
                self.aim.global.aimbot.multipoint = false;
                self.player.draw_skeleton = DrawMode::Color;
                self.player.draw_box = DrawMode::None;
                self.player.box_thickness = 1.0;
                self.player.snaplines = false;
            }
            2 => {
                // Recommended
                self.aim.global.aimbot.fov = 3.0;
                self.aim.global.aimbot.smooth = 6.0;
                self.aim.global.aimbot.hitchance = 100.0;
                self.aim.global.aimbot.backtrack = true;
                self.aim.global.aimbot.backtrack_ticks = 12;
                self.aim.global.aimbot.multipoint = true;
                self.aim.global.aimbot.multipoint_scale = 0.4;
                self.player.draw_skeleton = DrawMode::Color;
                self.player.draw_box = DrawMode::Color;
                self.player.box_thickness = 1.0;
                self.player.box_mode = BoxMode::Gap;
                self.player.snaplines = false;
            }
            3 => {
                // Blatant
                self.aim.global.aimbot.fov = 6.0;
                self.aim.global.aimbot.smooth = 3.0;
                self.aim.global.aimbot.hitchance = 100.0;
                self.aim.global.aimbot.backtrack = true;
                self.aim.global.aimbot.backtrack_ticks = 15;
                self.aim.global.aimbot.multipoint = true;
                self.aim.global.aimbot.multipoint_scale = 0.7;
                self.player.draw_box = DrawMode::Color;
                self.player.box_mode = BoxMode::Full;
                self.player.box_thickness = 1.5;
                self.player.snaplines = true;
                self.player.snapline_start = SnaplineStart::Bottom;
            }
            4 => {
                // Rage
                self.aim.global.aimbot.fov = 45.0;
                self.aim.global.aimbot.smooth = 1.0;
                self.aim.global.aimbot.hitchance = 100.0;
                self.aim.global.aimbot.backtrack = true;
                self.aim.global.aimbot.backtrack_ticks = 15;
                self.aim.global.aimbot.visibility_check = false;
                self.aim.global.aimbot.multipoint = true;
                self.aim.global.aimbot.multipoint_scale = 1.0;
                self.aim.global.triggerbot.enabled = true;
                self.aim.global.triggerbot.delay = 0..=0;
                self.player.draw_box = DrawMode::Color;
                self.player.box_mode = BoxMode::Full;
                self.player.box_thickness = 2.0;
                self.player.snaplines = true;
                self.player.snapline_start = SnaplineStart::Center;
                self.misc.bunnyhop = true;
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WeaponConfig {
    pub aimbot: AimbotConfig,
    pub rcs: RcsConfig,
    pub triggerbot: TriggerbotConfig,
}

impl WeaponConfig {
    pub fn enabled(enabled: bool) -> Self {
        let aimbot = AimbotConfig {
            enable_override: enabled,
            ..Default::default()
        };
        Self {
            aimbot,
            rcs: RcsConfig::default(),
            triggerbot: TriggerbotConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AimbotConfig {
    pub enable_override: bool,
    pub enabled: bool,
    pub mode: KeyMode,
    pub target_friendlies: bool,
    pub distance_adjusted_fov: bool,
    pub start_bullet: i32,
    pub visibility_check: bool,
    pub flash_check: bool,
    pub fov: f32,
    pub smooth: f32,
    pub bones: Vec<Bones>,
    pub targeting_mode: TargetingMode,
    pub prediction: bool,
    pub prediction_factor: f32,
    pub backtrack: bool,
    pub backtrack_ticks: u32,
    pub advanced_humanizer: bool,
    pub hitchance: f32,
    pub multipoint: bool,
    pub multipoint_scale: f32,
}

impl Default for AimbotConfig {
    fn default() -> Self {
        Self {
            enable_override: false,
            enabled: true,
            mode: KeyMode::Hold,
            target_friendlies: false,
            distance_adjusted_fov: true,
            start_bullet: 0,
            visibility_check: true,
            flash_check: true,
            fov: 2.5,
            smooth: 5.0,
            bones: vec![
                Bones::Head,
                Bones::Neck,
                Bones::Spine4,
                Bones::Spine3,
                Bones::Spine2,
                Bones::Spine1,
                Bones::Hip,
            ],
            targeting_mode: TargetingMode::Fov,
            prediction: false,
            prediction_factor: 1.0,
            backtrack: false,
            backtrack_ticks: 12,
            advanced_humanizer: true,
            hitchance: 100.0,
            multipoint: false,
            multipoint_scale: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RcsConfig {
    pub enable_override: bool,
    pub enabled: bool,
    pub smooth: f32,
}

impl Default for RcsConfig {
    fn default() -> Self {
        Self {
            enable_override: false,
            enabled: false,
            smooth: 0.3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum KeyMode {
    Hold,
    Toggle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum TargetingMode {
    Fov,
    Distance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TriggerbotConfig {
    pub enable_override: bool,
    pub enabled: bool,
    pub delay: RangeInclusive<u64>,
    pub shot_duration: u64,
    pub mode: KeyMode,
    pub flash_check: bool,
    pub scope_check: bool,
    pub velocity_check: bool,
    pub velocity_threshold: f32,
    pub head_only: bool,
}

impl Default for TriggerbotConfig {
    fn default() -> Self {
        Self {
            enable_override: false,
            enabled: false,
            delay: 100..=200,
            shot_duration: 200,
            mode: KeyMode::Hold,
            flash_check: true,
            scope_check: true,
            velocity_check: true,
            velocity_threshold: 100.0,
            head_only: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AimConfig {
    pub aimbot_hotkey: KeyCode,
    pub triggerbot_hotkey: KeyCode,
    pub global: WeaponConfig,
    pub weapons: HashMap<Weapon, WeaponConfig>,
}

impl Default for AimConfig {
    fn default() -> Self {
        let mut weapons = HashMap::new();
        for weapon in Weapon::iter() {
            weapons.insert(weapon, WeaponConfig::default());
        }

        Self {
            aimbot_hotkey: KeyCode::Mouse5,
            triggerbot_hotkey: KeyCode::Mouse4,
            global: WeaponConfig::enabled(true),
            weapons,
        }
    }
}

#[derive(Debug, Clone, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum DrawMode {
    None,
    Health,
    Color,
}

#[derive(Debug, Clone, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum BoxMode {
    Gap,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerConfig {
    pub enabled: bool,
    pub esp_hotkey: KeyCode,
    pub show_friendlies: bool,
    pub draw_box: DrawMode,
    pub box_mode: BoxMode,
    pub box_thickness: f32,
    pub box_visible_color: Color32,
    pub box_invisible_color: Color32,
    pub draw_skeleton: DrawMode,
    pub skeleton_visible_color: Color32,
    pub skeleton_invisible_color: Color32,
    pub skeleton_thickness: f32,
    pub head_circle: bool,
    pub health_bar: bool,
    pub armor_bar: bool,
    pub player_name: bool,
    pub weapon_icon: bool,
    pub snaplines: bool,
    pub snapline_color: Color32,
    pub snapline_start: SnaplineStart,
    pub tags: bool,
    pub visible_only: bool,
    pub sound: SoundConfig,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            esp_hotkey: KeyCode::X,
            show_friendlies: false,
            draw_box: DrawMode::Color,
            box_mode: BoxMode::Gap,
            box_thickness: 1.0,
            box_visible_color: Color32::WHITE,
            box_invisible_color: Color32::RED,
            draw_skeleton: DrawMode::Color,
            skeleton_visible_color: Color32::WHITE,
            skeleton_invisible_color: Color32::RED,
            skeleton_thickness: 1.0,
            head_circle: true,
            health_bar: true,
            armor_bar: true,
            player_name: true,
            weapon_icon: true,
            snaplines: false,
            snapline_color: Color32::WHITE,
            snapline_start: SnaplineStart::Bottom,
            tags: true,
            visible_only: false,
            sound: SoundConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SoundConfig {
    pub enabled: bool,
    pub footstep_diameter: f32,
    pub gunshot_diameter: f32,
    pub weapon_diameter: f32,
    pub fadeout_start: f32,
    pub fadeout_duration: f32,
    pub show_visible: bool,
}

impl Default for SoundConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            footstep_diameter: crate::constants::cs2::SOUND_ESP_FOOTSTEP_DIAMETER_DEFAULT,
            gunshot_diameter: crate::constants::cs2::SOUND_ESP_GUNSHOT_DIAMETER_DEFAULT,
            weapon_diameter: crate::constants::cs2::SOUND_ESP_WEAPON_DIAMETER_DEFAULT,
            fadeout_start: 1.0,
            fadeout_duration: 1.0,
            show_visible: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HudConfig {
    pub bomb_timer: bool,
    pub bomb_damage: bool,
    pub spectator_list: bool,
    pub fov_circle: bool,
    pub fov_arrows: bool,
    pub arrow_size: f32,
    pub arrow_radius: f32,
    pub keybind_list: bool,
    pub hitmarker: bool,
    pub hitmarker_color: Color32,
    pub bullet_tracers: bool,
    pub tracer_color: Color32,
    pub sniper_crosshair: bool,
    pub crosshair_color: Color32,
    pub dropped_weapons: bool,
    pub grenade_trails: bool,
    pub smoke_trail_color: Color32,
    pub molotov_trail_color: Color32,
    pub incendiary_trail_color: Color32,
    pub flash_trail_color: Color32,
    pub he_trail_color: Color32,
    pub decoy_trail_color: Color32,
    pub text_outline: bool,
    pub text_color: Color32,
    pub line_width: f32,
    pub font_size: f32,
    pub icon_size: f32,
    pub debug: bool,
}

impl Default for HudConfig {
    fn default() -> Self {
        Self {
            bomb_timer: true,
            bomb_damage: true,
            spectator_list: true,
            fov_circle: false,
            fov_arrows: true,
            arrow_size: 15.0,
            arrow_radius: 150.0,
            keybind_list: true,
            hitmarker: false,
            hitmarker_color: Color32::WHITE,
            bullet_tracers: false,
            tracer_color: Color32::from_rgba_unmultiplied(255, 255, 255, 100),
            sniper_crosshair: true,
            crosshair_color: Color32::WHITE,
            dropped_weapons: true,
            grenade_trails: true,
            smoke_trail_color: Color32::LIGHT_GRAY,
            molotov_trail_color: Color32::RED,
            incendiary_trail_color: Color32::ORANGE,
            flash_trail_color: Color32::WHITE,
            he_trail_color: Color32::DARK_GRAY,
            decoy_trail_color: Color32::PURPLE,
            text_outline: true,
            text_color: Colors::TEXT,
            line_width: 2.0,
            font_size: 16.0,
            icon_size: 20.0,
            debug: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RadarConfig {
    pub enabled: bool,
    pub scale: f32,
    pub size: f32,
    pub position: [f32; 2],
}

impl Default for RadarConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            scale: 1.0,
            size: 250.0,
            position: [50.0, 50.0],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UnsafeConfig {
    pub no_flash: bool,
    pub max_flash_alpha: f32,
    pub fov_changer: bool,
    pub desired_fov: u32,
    pub no_smoke: bool,
    pub change_smoke_color: bool,
    pub smoke_color: Color32,
    pub auto_accept: bool,
    pub bunnyhop: bool,
}

impl Default for UnsafeConfig {
    fn default() -> Self {
        Self {
            no_flash: false,
            max_flash_alpha: 127.0,
            fov_changer: false,
            desired_fov: 90,
            no_smoke: false,
            change_smoke_color: false,
            smoke_color: Color32::RED,
            auto_accept: false,
            bunnyhop: false,
        }
    }
}

pub static BASE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let sudo_user = std::env::var("SUDO_USER").unwrap_or_default();
    
    let base = if !sudo_user.is_empty() {
        PathBuf::from(format!("/home/{}", sudo_user)).join(".config")
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
            .unwrap_or_else(|| {
                std::env::current_exe()
                    .ok()
                    .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
                    .unwrap_or_else(|| PathBuf::from("."))
            })
    };

    let path = base.join("deadlocked");
    if !path.exists() {
        let _ = std::fs::create_dir_all(&path);
        
        // Fix permissions for SUDO_USER
        if !sudo_user.is_empty() {
            let _ = std::process::Command::new("chown")
                .arg("-R")
                .arg(format!("{}:{}", sudo_user, sudo_user))
                .arg(&path)
                .status();
        }
    }
    path
});

pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = BASE_PATH.join("configs");
    if !path.exists() {
        let _ = std::fs::create_dir_all(&path);
    }
    path
});

pub fn parse_config(path: &Path) -> Config {
    if !path.exists() || path.is_dir() {
        return Config::default();
    }

    let Ok(config_string) = read_to_string(path) else {
        return Config::default();
    };

    let config = toml::from_str(&config_string);
    if config.is_err() {
        log::warn!("config file invalid");
    } else if let Some(file_name) = path.file_name() {
        log::info!("loaded config {:?}", file_name);
    }
    config.unwrap_or_default()
}

pub fn write_config(config: &Config, path: &Path) {
    let out = toml::to_string(&config).unwrap();
    let _ = std::fs::write(path, out);
}

pub fn delete_config(path: &Path) {
    if !path.exists() {
        return;
    }

    if std::fs::remove_file(path).is_ok()
        && let Some(file_name) = path.file_name()
    {
        log::info!("deleted config {:?}", file_name);
    }
}

pub fn available_configs() -> Vec<PathBuf> {
    let mut files = Vec::with_capacity(8);
    let Ok(dir) = std::fs::read_dir::<&Path>(CONFIG_PATH.as_ref()) else {
        return files;
    };

    for path in dir {
        let Ok(file) = path else {
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
        if !file_name.ends_with(".toml") {
            continue;
        }
        files.push(file.path())
    }
    if files.is_empty() {
        let path = CONFIG_PATH.join(DEFAULT_CONFIG_NAME);
        write_config(&Config::default(), &path);
        files.push(path);
    }
    files
}
