# Android local setup (Pi-Bolt sandbox)

No system cargo exists here and none can be installed normally — the sandbox is
Android/Bionic with a bundled glibc-2.44 port. A Debian-based toolchain lives in
`~/toolchains/` (rust 1.100 + gcc 14 + glibc headers). **Do not rebuild it; just use it.**

## Cold start

```sh
# frontend dev server (hot reload) — http://localhost:4321
npm install   # first time only
npm run dev

# backend — http://localhost:8080 (serves dist/ + /api)
. ~/toolchains/env.sh
cd lacq && $CARGO build --release && cd ..
npm run build            # frontend → dist/ (backend serves it)
ln -sfn ~/projects/lacoquille/dist lacq/dist
~/toolchains/bin/lacq-server
```

4321 proxies `/api` → 8080; 8080 serves everything by itself. Verify: `curl localhost:8080/health`.

## Rules (each one caused a real failure)

- **`LD_LIBRARY_PATH` order is critical**: toolbox `$U/lib` (custom glibc) must come
  **before** `debian-root/...`. Debian's `libc.so.6` shadowing it = instant SIGBUS.
  This is already correct in `~/toolchains/env.sh` and `~/toolchains/bin/lacq-server` —
  don't reorder, don't prepend Debian paths in ad-hoc commands.
- **Never run downloaded/static or musl binaries** — only glibc-dynamic ELF binaries
  work (the toolbox exec supervisor rewrites interp paths; static ones get EACCES).
- **`reqwest` must keep `default-features = false`** (Cargo.toml) — default-tls pulls
  OpenSSL, which doesn't exist here. Compile-time fix already applied; keep it.
- Crates with `build.rs` are fine (build scripts run like any binary). No `pkg-config`
  available; nothing currently needs it.
- The exec supervisor + loader quirks mean errors can look cryptic (`Bus error`,
  EACCES, "cannot execute") — first check `LD_LIBRARY_PATH`, then whether the binary
  is glibc-dynamic.
