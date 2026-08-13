# fullstack_rust test demo

Temporary, throwaway fixture used to test the `fullstack_rust` live-demo
pipeline (and Phase 12's on-demand screenshot capture) on
[cv_website](https://github.com/ramondw2000/cv_website). Not a real project —
safe to delete once testing is done.

A minimal, dependency-free Rust HTTP server (`std::net::TcpListener` only)
that:

- Reads the `PORT` env var and binds to it — the runner's whole launch
  contract for `fullstack_rust` demos.
- Serves one self-contained HTML page at `/` (inline CSS/JS, no external
  assets) with a button that calls a small JSON API at `api/hello` — both
  paths are relative, since the demo runs behind a path-prefixed reverse
  proxy (`/sessions/:id/app/...`) and a root-absolute URL would 404.

Releases are built by `.github/workflows/release.yml` on every `v*` tag push,
producing a bare Linux binary named `demo-server-linux` (no archive — the
runner executes release assets directly after a `chmod`, no extraction step).
