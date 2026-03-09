use std::{
    backtrace::Backtrace,
    collections::HashMap,
    panic::{self, PanicHookInfo},
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use utils::log;

use crate::os::NO_REQUESTS;

pub fn install_crash_handler() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_hook_info| {
        crash_handler(panic_hook_info);
        default_hook(panic_hook_info);
    }));
}

pub const TIMEOUT_DURATION: Duration = Duration::from_secs(2);
static SENT_REPORT: AtomicBool = AtomicBool::new(false);
fn crash_handler(panic_info: &PanicHookInfo) {
    if *NO_REQUESTS {
        return;
    }
    if SENT_REPORT.swap(true, Ordering::SeqCst) {
        return;
    }

    let hwid = hwid();
    let mut stacktrace = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
        s.clone()
    } else {
        "explicit panic".to_string()
    };

    if let Some(location) = panic_info.location() {
        stacktrace.push_str(&format!(" at {}:{}", location.file(), location.line()));
    }

    stacktrace.push_str("\n\nStacktrace:\n");
    stacktrace.push_str(&Backtrace::force_capture().to_string());

    let json = serde_json::json!({"hwid": hwid,"stacktrace": stacktrace});

    let client_config = ureq::config::Config::builder()
        .timeout_global(Some(TIMEOUT_DURATION))
        .build();
    let client = ureq::Agent::new_with_config(client_config);
    let _ = client
        .post("https://deadlocked.holyhades64.workers.dev/stacktrace")
        .send_json(json);

    log::info!("crash reported");
}

const UNKNOWN: &str = "unknown";

pub fn info() {
    if *NO_REQUESTS {
        return;
    };

    let hwid = hwid();
    let kernel = std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or(UNKNOWN.to_owned())
        .trim()
        .to_owned();
    let distro = distro();
    let desktop = desktop();
    let rust_version = rust_version();
    let git_commit = git_commit();

    let json = serde_json::json!({
        "hwid": hwid,
        "kernel": kernel,
        "distro": distro,
        "desktop": desktop,
        "rust_version": rust_version,
        "git_commit": git_commit
    });
    let _ = ureq::post("https://deadlocked.holyhades64.workers.dev/misc").send_json(json);
}

fn hwid() -> String {
    std::fs::read_to_string("/etc/machine-id")
        .unwrap_or(UNKNOWN.to_owned())
        .trim()
        .to_owned()
}

fn distro() -> String {
    let Ok(content) = std::fs::read_to_string("/etc/os-release") else {
        return UNKNOWN.to_owned();
    };
    let mut info = HashMap::new();
    for line in content.lines() {
        let mut parts = line.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            info.insert(key, value.trim_matches('"'));
        }
    }
    if let Some(name) = info.get("PRETTY_NAME") {
        return (*name).to_owned();
    } else if let Some(name) = info.get("NAME") {
        return (*name).to_owned();
    }
    UNKNOWN.to_owned()
}

fn desktop() -> String {
    let de_vars = [
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
        "GDMSESSION",
        "GNOME_DESKTOP_SESSION_ID",
    ];

    for var in &de_vars {
        if let Ok(val) = std::env::var(var)
            && !val.is_empty()
        {
            return val;
        }
    }

    UNKNOWN.to_string()
}

fn rust_version() -> String {
    Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or(UNKNOWN.to_owned())
}

fn git_commit() -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or(UNKNOWN.to_owned())
}
