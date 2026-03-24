use std::sync::Arc;

use crossbeam::channel::{bounded, unbounded};
use parking_lot::RwLock;
use utils::{
    log::{self, Logger, LoggerOptions},
    sync::Mutex,
};

use crate::{
    data::Data,
    os::mouse::check_uinput,
    parser::parse_maps,
    ui::{app::App, grenades::read_grenades},
};

mod config;
mod constants;
mod cs2;
mod data;
mod game;
mod math;
mod message;
mod os;
mod parser;
mod router;
mod ui;

#[cfg(not(target_os = "linux"))]
compile_error!("only linux is supported.");

fn main() {
    Logger::install(
        LoggerOptions::default()
            .file("deadlocked.log")
            .debug(true)
            .truncate(true)
            .module(module_path!()),
    );

    let args: Vec<String> = std::env::args().collect();
    os::crash::install_crash_handler();

    if !check_uinput() {
        return;
    }

    let force_reparse = args.iter().any(|arg| arg == "--force-reparse");
    let use_system_binary = args.iter().any(|arg| arg == "--local-s2v");
    let force_x11 = args.iter().any(|arg| arg == "--x11");
    let force_wayland = args.iter().any(|arg| arg == "--wayland");

    let wayland_available = std::env::var("WAYLAND_DISPLAY").is_ok();
    let x11_available = std::env::var("DISPLAY").is_ok();
    let auto_x11_fallback = !force_wayland && wayland_available && x11_available;

    if force_x11 || auto_x11_fallback {
        unsafe { std::env::remove_var("WAYLAND_DISPLAY") };
        if auto_x11_fallback && !force_x11 {
            log::info!(
                "wayland detected; using x11/xwayland backend for reliable overlay positioning (pass --wayland to disable)"
            );
        }
    } else if wayland_available && !x11_available {
        log::info!(
            "running on native wayland backend; compositor may ignore absolute window positioning"
        );
    }
    spawn_with_crash_handler(move || {
        parse_maps(force_reparse, use_system_binary);
    });

    let (tx, rx) = unbounded();
    let (tx_gui, rx_gui) = bounded(16);
    let (tx_game, rx_game) = bounded(16);
    let data = Arc::new(RwLock::new(Data::default()));
    let data_game = data.clone();
    let grenades = Arc::new(Mutex::new(read_grenades()));
    let grenades_game = grenades.clone();

    spawn_with_crash_handler(move || {
        router::router(rx, tx_gui, tx_game);
    });

    let tx_game = tx.clone();
    spawn_with_crash_handler(move || {
        game::GameManager::new(tx_game, rx_game, data_game, grenades_game).run();
    });

    let event_loop = match winit::event_loop::EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(err) => {
            log::error!("failed to create event loop: {err}");
            return;
        }
    };
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::new(tx, rx_gui, data, grenades);
    if let Err(err) = event_loop.run_app(&mut app) {
        log::error!("event loop failed: {err}");
    }
}

fn spawn_with_crash_handler<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    std::thread::spawn(move || {
        os::crash::install_crash_handler();
        f();
    });
}
