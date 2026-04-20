//! End-to-end integration test for the HTTP API.
//!
//! Spawns the real HTTP server on a random port and exercises the full
//! creative workflow: list presets -> generate demo -> create session ->
//! render preview -> parameter sweep -> preset patch.

use std::net::TcpListener;
use std::time::Duration;

use serde_json::{json, Value};

fn find_free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

fn api_url(port: u16, path: &str) -> String {
    format!("http://127.0.0.1:{port}{path}")
}

fn post_tool(port: u16, tool: &str, params: Value) -> Value {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(api_url(port, &format!("/api/tools/{tool}")))
        .header("Authorization", "Bearer test-integration-key")
        .header("Content-Type", "application/json")
        .json(&params)
        .send()
        .unwrap();
    resp.json().unwrap()
}

#[test]
fn test_http_creative_workflow_end_to_end() {
    let port = find_free_port();
    let tmp = tempfile::tempdir().unwrap();
    let preset_dir = tmp.path().join("presets");
    let runtime_dir = tmp.path().join("runtime");

    // Spawn server in background thread
    let preset_dir_clone = preset_dir.clone();
    let runtime_dir_clone = runtime_dir.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(state_space_music_box::http_server::start_http_server(
            preset_dir_clone,
            runtime_dir_clone,
            "test-integration-key".to_string(),
            port,
        ))
        .ok();
    });

    // Wait for server to start
    std::thread::sleep(Duration::from_millis(500));

    // 1. Health check
    let client = reqwest::blocking::Client::new();
    let health: Value = client
        .get(api_url(port, "/api/health"))
        .send()
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(health["success"], true);
    assert_eq!(health["data"]["status"], "ok");

    // 2. List tools
    let tools: Vec<Value> = client
        .get(api_url(port, "/api/tools"))
        .send()
        .unwrap()
        .json()
        .unwrap();
    assert!(
        tools.len() >= 30,
        "expected at least 30 tools, got {}",
        tools.len()
    );

    // 3. Auth failure
    let no_auth: Value = client
        .post(api_url(port, "/api/tools/list_presets"))
        .header("Content-Type", "application/json")
        .json(&json!({}))
        .send()
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(no_auth["success"], false);

    // 4. List presets
    let presets = post_tool(port, "list_presets", json!({}));
    assert_eq!(presets["success"], true);
    let preset_list = presets["data"].as_array().unwrap();
    assert!(preset_list.iter().any(|p| p["name"] == "demo"));

    // 5. Generate demo
    let demo = post_tool(port, "generate_demo", json!({"seed": 42}));
    assert_eq!(demo["success"], true);
    assert!(demo["data"]["note_count"].as_u64().unwrap() > 0);
    assert!(demo["data"]["audio_sample_count"].as_u64().unwrap() > 0);

    // 6. Create session
    let session = post_tool(
        port,
        "session_create",
        json!({"display_name": "Integration Test", "preset": "demo", "seed": 7}),
    );
    assert_eq!(session["success"], true);
    let session_id = session["data"]["session_id"].as_str().unwrap().to_string();
    assert!(!session_id.is_empty());

    // 7. Render preview
    let preview = post_tool(
        port,
        "session_render_preview",
        json!({"session_id": session_id}),
    );
    assert_eq!(preview["success"], true);

    // 8. Parameter sweep
    let sweep = post_tool(
        port,
        "parameter_sweep",
        json!({"preset": "demo", "seeds": [1, 2, 3, 4, 5]}),
    );
    assert_eq!(sweep["success"], true);
    let entries = sweep["data"]["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 5);
    let ranked = sweep["data"]["ranked_seeds"].as_array().unwrap();
    assert_eq!(ranked.len(), 5);

    // 9. Session list
    let sessions = post_tool(port, "session_list", json!({}));
    assert_eq!(sessions["success"], true);
    let session_list = sessions["data"].as_array().unwrap();
    assert_eq!(session_list.len(), 1);

    // 10. Audit list
    let audit = post_tool(port, "audit_list", json!({}));
    assert_eq!(audit["success"], true);

    // 11. Sweep list
    let sweeps = post_tool(port, "sweep_list", json!({}));
    assert_eq!(sweeps["success"], true);
    let sweep_list = sweeps["data"].as_array().unwrap();
    assert_eq!(sweep_list.len(), 1);

    // 12. Unknown tool returns error
    let unknown = post_tool(port, "nonexistent_tool", json!({}));
    assert_eq!(unknown["success"], false);
    assert!(unknown["error"].as_str().unwrap().contains("unknown tool"));
}
