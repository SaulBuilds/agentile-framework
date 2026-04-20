//! Demonstrate using the HTTP API from Rust (conceptual example).
//!
//! This example shows the curl equivalents for common operations.
//! To actually run HTTP requests from Rust, you'd add reqwest as a dependency.
//!
//! Start the server first:
//!   cargo run -- http --port 3001 --api-key my-secret-key
//!
//! Then in another terminal, run these curl commands:

fn main() {
    println!("=== state-space-music-box HTTP API Examples ===\n");

    println!("1. Start the server:");
    println!("   cargo run -- http --port 3001 --api-key my-secret-key\n");

    println!("2. Health check:");
    println!("   curl http://localhost:3001/api/health\n");

    println!("3. List available tools:");
    println!("   curl http://localhost:3001/api/tools\n");

    println!("4. List presets:");
    println!(
        r#"   curl -X POST http://localhost:3001/api/tools/list_presets \
     -H "Authorization: Bearer my-secret-key" \
     -H "Content-Type: application/json" \
     -d '{{}}'"#
    );
    println!();

    println!("5. Generate a demo composition:");
    println!(
        r#"   curl -X POST http://localhost:3001/api/tools/generate_demo \
     -H "Authorization: Bearer my-secret-key" \
     -H "Content-Type: application/json" \
     -d '{{"seed": 42}}'"#
    );
    println!();

    println!("6. Create a session:");
    println!(
        r#"   curl -X POST http://localhost:3001/api/tools/session_create \
     -H "Authorization: Bearer my-secret-key" \
     -H "Content-Type: application/json" \
     -d '{{"display_name": "My Session", "preset": "demo", "seed": 1}}'"#
    );
    println!();

    println!("7. Render a session preview:");
    println!(
        r#"   curl -X POST http://localhost:3001/api/tools/session_render_preview \
     -H "Authorization: Bearer my-secret-key" \
     -H "Content-Type: application/json" \
     -d '{{"session_id": "<session-id-from-step-6>"}}'"#
    );
    println!();

    println!("8. Create a harness plan (agent workflow):");
    println!(
        r#"   curl -X POST http://localhost:3001/api/tools/harness_plan \
     -H "Authorization: Bearer my-secret-key" \
     -H "Content-Type: application/json" \
     -d '{{"role": "session_dj", "prompt": "set tempo to 140 and render a preview", "session_id": "<session-id>"}}'"#
    );
    println!();

    println!("9. Schedule an unattended job:");
    println!(
        r#"   curl -X POST http://localhost:3001/api/tools/job_validate \
     -H "Authorization: Bearer my-secret-key" \
     -H "Content-Type: application/json" \
     -d '{{"backend": "local_cli", "role": "session_dj", "prompt": "render and evaluate", "session_id": "<session-id>", "retry_limit": 1}}'"#
    );
    println!();

    println!("All responses follow the format:");
    println!(r#"   {{"success": true, "data": {{...}}}}"#);
    println!(r#"   {{"success": false, "error": "description"}}"#);
}
