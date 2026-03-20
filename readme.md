# deadlocked v4.6.8

A high-performance, external CS2 aimbot and ESP framework for Linux, built with stealth and modern anti-cheat bypasses in mind.

## Setup

```bash
./setup.sh
# Restart your machine (required for uinput/permissions)
git clone https://github.com/Domanffe/deadlocked
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Core Requirements
- **uinput:** Kernel module must be loaded for stealthy mouse input.
- **Native Steam:** Flatpak versions are not supported due to process sandboxing.
- **Permissions:** Read/Write access to /dev/uinput and memory access (handled by setup.sh).

## Running

```bash
./run.sh
```

> [!IMPORTANT]
> On the first run or after game updates, the system will perform an asynchronous VPK/Map parse to build BVH trees for instantaneous visibility checks. This process is resource-intensive; it is recommended to let it finish before joining a match.

## Features

### Aimbot (Legit & Professional)
- **Advanced Humanizer:** Bezier curve interpolation with Gaussian noise injection to simulate natural hand tremors and bypass behavioral analysis (VACNet 3.0).
- **Sub-tick Prediction:** Linear extrapolation based on target velocity for perfect leading.
- **Dynamic Bone Selection:** Smart scanning of multiple hitboxes; automatically switches to the nearest visible point.
- **Hitchance:** Configurable probability logic to simulate human error and maintain natural statistics.
- **Multipoint:** Scans hitbox edges to find exposed areas when the center is behind cover.
- **Backtrack:** Ability to target enemy positions up to 200ms in the past (15 ticks) to compensate for latency.

### Visuals (External Overlay)
- **Instantaneous Visibility:** Leverages local BVH geometry maps for real-time color changes (no delay from game engine bits).
- **Skeleton ESP:** Precise joint rendering with configurable thickness and head-circle scale.
- **Out-of-FOV Arrows:** 360-degree tactical indicators pointing to off-screen enemies.
- **Snaplines:** Lines from any screen edge to targets for quick target acquisition.
- **Sound ESP:** Visual pulse indicators at the origin of footsteps and gunshots.
- **Volumetric Smoke Check:** Mathematical intersection checks to prevent aiming through smoke clouds.

### Interface & System
- **Full Localization:** Native support for Russian and English languages.
- **Modern GUI:** Sleek dark-mode interface with rounded corners, pill-style navigation, and accent color customization.
- **Config Presets:** 5 pre-tuned tiers from Legit to Rage for instant setup.
- **Web Radar:** See the entire map and all players on any device via a browser.
- **Keybind List:** Real-time HUD overlay showing active features and their hotkey status.

### Misc & Unsafe
- **Auto-Accept:** Automatically joins matches when found.
- **Bunnyhop:** Perfect jump timing when holding spacebar.
- **Bomb Stats:** Timer and Damage Predictor (calculates lethal distance vs armor).
- **Visual Modifiers:** No Flash (configurable alpha), FOV Changer, and custom Smoke colors.

## FAQ

### Where are my configs saved?
Configs are managed in the Config tab. Use Presets for quick setup, or click "Save to Current Profile" to persist your manual adjustments. Files are stored in $HOME/.config/deadlocked/configs.

### Wayland Support
Native Wayland is supported. For Hyprland, add these rules to your config for a perfect overlay:

```conf
windowrulev2 = float, title:^(deadlocked_overlay)$
windowrulev2 = pin, title:^(deadlocked_overlay)$
windowrulev2 = noinput, title:^(deadlocked_overlay)$
windowrulev2 = noblur, title:^(deadlocked_overlay)$
windowrulev2 = opaque, title:^(deadlocked_overlay)$
windowrulev2 = size 100% 100%, title:^(deadlocked_overlay)$
```

---

> [!CAUTION]
> **Play Smart.** VAC Live and VACNet 3.0 are highly effective at detecting robotic movements and suspicious statistics. Use a low FOV, high smoothing, and keep your HS% within reasonable limits.