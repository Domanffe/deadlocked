#![allow(unused)]
use egui::Color32;
use serde::{Deserialize, Serialize};

pub struct Colors;

impl Colors {
    pub const BACKDROP: Color32 = Color32::from_rgb(10, 10, 10);
    pub const BASE: Color32 = Color32::from_rgb(17, 17, 17);
    pub const HIGHLIGHT: Color32 = Color32::from_rgb(37, 37, 37);
    pub const SUBTEXT: Color32 = Color32::from_rgb(161, 161, 170);
    pub const TEXT: Color32 = Color32::from_rgb(237, 237, 237);

    pub const RED: Color32 = Color32::from_rgb(255, 105, 180);
    pub const ORANGE: Color32 = Color32::from_rgb(249, 115, 22);
    pub const YELLOW: Color32 = Color32::from_rgb(234, 179, 8);
    pub const GREEN: Color32 = Color32::from_rgb(20, 184, 166);
    pub const TEAL: Color32 = Color32::from_rgb(20, 184, 166);
    pub const BLUE: Color32 = Color32::from_rgb(0, 169, 224);
    pub const PURPLE: Color32 = Color32::from_rgb(139, 92, 246);

    pub const ACCENT: Color32 = Color32::from_rgb(0, 169, 224);
    pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
    pub const GRAY: Color32 = Color32::from_rgb(113, 113, 122);

    pub const ACCENT_COLORS: [(&str, Color32); 7] = [
        ("Electric Blue", Self::BLUE),
        ("Teal", Self::TEAL),
        ("Warm Orange", Self::ORANGE),
        ("Neon Pink", Self::RED),
        ("Sage", Color32::from_rgb(132, 169, 140)),
        ("Mocha", Color32::from_rgb(139, 94, 60)),
        ("Violet", Self::PURPLE),
    ];
}
