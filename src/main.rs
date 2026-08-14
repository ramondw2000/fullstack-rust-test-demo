use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// Deliberately relative paths only (no leading "/") -- the demo is served
// behind a path-prefixed reverse proxy (/sessions/:id/app/...), and a
// root-absolute URL would resolve against the proxy host itself instead of
// staying under the session prefix.
const INDEX_HTML: &str = r##"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<title>fullstack_rust test demo</title>
<style>
  body { font-family: monospace; background: #0a0a0c; color: #eee; display: flex; align-items: center; justify-content: center; height: 100vh; margin: 0; }
  .card { text-align: center; }
  button { margin-top: 1rem; padding: 0.5rem 1rem; font-family: inherit; cursor: pointer; }
  #out { margin-top: 1rem; color: #b268f3; min-height: 1.5em; }
</style>
</head>
<body>
  <div class="card">
    <h1>fullstack_rust test demo</h1>
    <p>temporary fixture &mdash; safe to delete</p>
    <button onclick="ping()">call backend</button>
    <div id="out"></div>
  </div>
  <script>
    async function ping() {
      const res = await fetch("api/hello");
      const data = await res.json();
      document.getElementById("out").textContent = data.message;
    }
  </script>
</body>
</html>"##;

fn handle(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    let n = match stream.read(&mut buf) {
        Ok(n) => n,
        Err(_) => return,
    };
    let req = String::from_utf8_lossy(&buf[..n]);
    let path = req
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    let (status, content_type, body): (&str, &str, String) = if path == "/" {
        ("200 OK", "text/html; charset=utf-8", INDEX_HTML.to_string())
    } else if path == "/api/hello" {
        (
            "200 OK",
            "application/json",
            "{\"message\":\"hello from the rust backend\"}".to_string(),
        )
    } else if path == "/api/env-check" {
        // cv_website GH issue Phase 37 verification: proves an admin-supplied
        // custom env file actually reaches this spawned process, without
        // needing an interactive terminal/PTY (unlike cli_tool demos, this
        // server can just be asked directly over HTTP).
        let value = env::var("PHASE37_TEST_VAR")
            .unwrap_or_else(|_| "unset".to_string())
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        (
            "200 OK",
            "application/json",
            format!("{{\"phase37TestVar\":\"{value}\"}}"),
        )
    } else {
        ("404 Not Found", "text/plain", "not found".to_string())
    };

    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len(),
    );
    let _ = stream.write_all(response.as_bytes());
}

fn main() {
    // The runner's whole launch contract for fullstack_rust: whatever this
    // binds to is expected to read PORT (see apps/runner/src/process-launcher.ts).
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8787);
    let listener = TcpListener::bind(("0.0.0.0", port)).expect("bind failed");
    for stream in listener.incoming().flatten() {
        thread::spawn(|| handle(stream));
    }
}
