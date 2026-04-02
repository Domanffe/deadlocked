# deadlocked-x v4.7.2

A high-performance, external CS2 aimbot and ESP framework for Linux, built with stealth and modern anti-cheat bypasses in mind.

## Fork Credits

This project is based on the original `deadlocked` by `avitran0`.
Upstream repository: https://github.com/avitran0/deadlocked

This fork continues development independently while respecting and crediting the original work.

## License

This project is licensed under **GNU GPLv3**.
When distributing binaries or modified versions, keep the license terms and provide access to corresponding source code.

See: [`license`](license)

## Setup

### Recommended: Download Release (Easiest)

Releases are the easiest way to get started (no `git clone` needed):

1. Open Releases: https://github.com/Domanffe/deadlocked/releases
2. Download the latest Linux archive.
3. Extract it and run:

```bash
chmod +x deadlocked
./deadlocked
```

> [!NOTE]
> Release archive is named `deadlocked-x-linux.tar.gz` and contains `deadlocked`, `license`, and `lib/libmapdata.so` (see `.github/workflows/release.yml`).
> `run.sh` and `setup.sh` are for source workflow.

### From Source (Development)

```bash
git clone https://github.com/Domanffe/deadlocked
cd deadlocked
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
./setup.sh
# Restart your machine (required for uinput/permissions)
```

### Core Requirements
- **uinput:** Kernel module must be loaded for stealthy mouse input.
- **Native Steam:** Flatpak versions are not supported due to process sandboxing.
- **Permissions:** Read/Write access to /dev/uinput and memory access (handled by setup.sh).
- **Platform Notes:** For distro-specific setup notes, see [`os-setup.md`](os-setup.md).

## Running

```bash
./deadlocked   # if you downloaded a Release
# or
./run.sh   # if you cloned the repository
```

> [!IMPORTANT]
> Geometry now loads on demand when the current map changes.
> The preferred path is live runtime mapdata, with fallback to cached `.bvh` data and then current-map `Source2Viewer` recovery if available.
> `--force-reparse` and `--local-s2v` still exist, but they now affect recovery for the current map instead of triggering a full startup parse of all maps.

## Features

### Aimbot (Legit & Professional)
- **Advanced Humanizer:** Bezier curve interpolation with Gaussian noise injection to simulate natural hand tremors and bypass behavioral analysis (VACNet 3.0).
- **Sub-tick Prediction:** Linear extrapolation based on target velocity for perfect leading.
- **Dynamic Bone Selection:** Smart scanning of multiple hitboxes; automatically switches to the nearest visible point.
- **Hitchance:** Configurable probability logic to simulate human error and maintain natural statistics.
- **Multipoint:** Scans hitbox edges to find exposed areas when the center is behind cover.
- **Backtrack:** Ability to target enemy positions up to 200ms in the past (15 ticks) to compensate for latency.

### Visuals (External Overlay)
- **Instantaneous Visibility:** Uses live runtime geometry when available, with cached BVH fallback for stable real-time visibility checks.
- **Skeleton ESP:** Precise joint rendering with configurable thickness and head-circle scale.
- **Out-of-FOV Arrows:** 360-degree tactical indicators pointing to off-screen enemies.
- **Snaplines:** Lines from any screen edge to targets for quick target acquisition.
- **Sound ESP:** Visual pulse indicators at the origin of footsteps and gunshots.
- **Volumetric Smoke Check:** Mathematical intersection checks to prevent aiming through smoke clouds.

### Interface & System
- **Full Localization:** Native support for Russian and English languages.
- **Modern GUI:** Sleek dark-mode interface with rounded corners, pill-style navigation, and accent color customization.
- **Config Presets:** 5 pre-tuned tiers from Legit to Rage for instant setup.
- **Overlay Radar:** Lightweight in-overlay radar with configurable size, scale, and position.
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
Wayland can work, but when both X11 and Wayland are available the app prefers X11/XWayland by default for more reliable overlay positioning.
Use `--wayland` to force native Wayland, or `--x11` to force X11/XWayland.

For Hyprland, these rules can still help the overlay behave more predictably:

```conf
windowrulev2 = float, title:^(deadlocked_overlay)$
windowrulev2 = pin, title:^(deadlocked_overlay)$
windowrulev2 = noinput, title:^(deadlocked_overlay)$
windowrulev2 = noblur, title:^(deadlocked_overlay)$
windowrulev2 = opaque, title:^(deadlocked_overlay)$
windowrulev2 = size 100% 100%, title:^(deadlocked_overlay)$
```

`setup.sh` only adds the `no_blur` rule automatically. If you want the full behavior above, add the remaining rules manually.

---

> [!CAUTION]
> **Play Smart.** VAC Live and VACNet 3.0 are highly effective at detecting robotic movements and suspicious statistics. Use a low FOV, high smoothing, and keep your HS% within reasonable limits.
