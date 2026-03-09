use crate::os::crash::TIMEOUT_DURATION;

pub fn add_kill(steamid: u64, display_name: String) {
    std::thread::spawn(move || {
        let json =
            serde_json::json!({"steam_id": steamid.to_string(), "display_name": display_name});

        let client_config = ureq::config::Config::builder()
            .timeout_global(Some(TIMEOUT_DURATION))
            .build();
        let client = ureq::Agent::new_with_config(client_config);
        let _ = client
            .post("https://deadlocked.holyhades64.workers.dev/kill")
            .send_json(json);
    });
}
