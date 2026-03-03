#![allow(unused)]
use egui::Color32;
use serde::{Deserialize, Serialize};

pub struct Colors;

impl Colors {
    pub const BACKDROP: Color32 = Color32::from_rgb(18, 18, 24); // Darker side panel
    pub const BASE: Color32 = Color32::from_rgb(26, 26, 34);     // Main window background
    pub const HIGHLIGHT: Color32 = Color32::from_rgb(45, 45, 60); // Hover states
    pub const SUBTEXT: Color32 = Color32::from_rgb(160, 160, 170); // Slightly bluer grey
    pub const TEXT: Color32 = Color32::from_rgb(240, 240, 245);
    
    pub const RED: Color32 = Color32::from_rgb(235, 87, 87); // Modern flat red
    pub const ORANGE: Color32 = Color32::from_rgb(242, 153, 74);
    pub const YELLOW: Color32 = Color32::from_rgb(242, 201, 76);
    pub const GREEN: Color32 = Color32::from_rgb(111, 207, 151);
    pub const TEAL: Color32 = Color32::from_rgb(86, 204, 242);
    pub const BLUE: Color32 = Color32::from_rgb(84, 160, 255);
    pub const PURPLE: Color32 = Color32::from_rgb(187, 107, 217);

    pub const ACCENT: Color32 = Color32::from_rgb(84, 160, 255); // Primary brand color
    pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
    pub const GRAY: Color32 = Color32::from_rgb(130, 130, 130);

    pub const ACCENT_COLORS: [(&str, Color32); 7] = [
        ("Red", Self::RED),
        ("Orange", Self::ORANGE),
        ("Yellow", Self::YELLOW),
        ("Green", Self::GREEN),
        ("Teal", Self::TEAL),
        ("Blue", Self::BLUE),
        ("Purple", Self::PURPLE),
    ];
}
