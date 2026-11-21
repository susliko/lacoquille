# Screencasts: analyze shared videos, record our own

Status after the 2026-11-21 setup session. Analyzing screencasts **works
today**. Recording the phone's own screen is **not possible from the sandbox**
— it needs a Pi-Bolt app-level feature (or a PC with adb/scrcpy).

## Analyzing a screencast (works)

### 1. User shares a video file

`/sdcard/Download` is readable **and writable** from the sandbox, so any
Android share target (Quick Share, Telegram, browser) that lands the file in
`Download/` is reachable. Just tell the user to share the screencast to
Downloads, then:

```sh
screencast-frames /sdcard/Download/myclip.mp4 [fps]     # default 1 fps
```

`~/.local/bin/screencast-frames` extracts numbered frames (scaled to ≤720px
wide) plus 4×4 contact sheets into `<video>-frames/`. `read` the
`sheet-*.png` files — they arrive as images in the conversation. Zoom into
individual `frame-*.png` when something needs a closer look.

### 2. User shares a YouTube (or yt-dlp-supported site) link

`yt-dlp` (pip-installed, `~/.local/bin`) + ffmpeg handle it:

```sh
yt-dlp -f "bv*[height<=720][ext=mp4]+ba[ext=m4a]/b[height<=720]" \
  --no-playlist -o /tmp/clip.mp4 "<url>"
screencast-frames /tmp/clip.mp4
```

Or skip ffmpeg entirely for a single moment: `fetch_content` with the YouTube
URL + `timestamp`/`frames` extracts frames directly (proven working).

### 3. fetch_content video mode (built-in)

The harness's `fetch_content` analyzes videos when given a **`file://` URL**
(plain paths do NOT work — must be `file:///...`). Works for local files and
YouTube; YouTube needs yt-dlp + ffmpeg, which are now installed.

## What was installed (the missing pieces)

Everything runs through the existing toolbox mechanism: Debian trixie arm64
debs unpacked into a prefix, executed via the kernel-side glibc exec
supervisor (same as the gcc toolchain — see `docs/android-local.md`).

- **ffmpeg 7.1.5 + ffprobe** — `~/toolchains/media-root/` (231-package Debian
  dependency closure, 455 MB unpacked; original debs cached in
  `~/toolchains/media-debs/`). Reproducible:
  `python3 ~/toolchains/media-setup/fetch-ffmpeg-closure.py`.
- **PATH shims** — `~/.local/bin/{ffmpeg,ffprobe}` (symlinks to
  `ffmpeg-bin`) set `LD_LIBRARY_PATH` in the required order (toolbox libc
  first — never let Debian's libc shadow it) and exec the real binary.
  Note: after (re)unpacking, symlink alternatives-managed libs into the main
  libdir: `cd ~/toolchains/media-root/usr/lib/aarch64-linux-gnu && \
  find . -mindepth 2 -name "*.so*" -exec ln -sf {} $(basename {}) \;`
  (covers `pulseaudio/`, `blas/`, `lapack/` subdirs).
- **yt-dlp** — `pip3 install yt-dlp`.
- **screencast-frames** — `~/.local/bin/screencast-frames` (see above).

## Recording our own screencast (the remaining gap)

Tried and ruled out from the sandbox:

- `/system/bin/screenrecord` → `ERROR: no display` (app UID has no display
  access; needs shell UID via adb or MediaProjection from the app itself).
- `/dev/graphics/fb0` → doesn't exist on modern Android (DRM-only display).
- WebView DevTools CDP screencast of the in-app preview → no devtools
  abstract socket exists (WebView debugging is disabled in release builds).
- `preview_screenshot` covers only the in-app preview and is manual — fine
  for a few stills of the lacoquille app, not for a real screencast.

Options, in order of usefulness:

1. **Pi-Bolt app feature (the real fix)** — expose a `screen_record` tool /
   extension host bridge using Android's MediaProjection API (user consent
   dialog, MediaCodec → mp4 into `/sdcard/Download`). Extensions are plain
   Node processes, so the bridge must live in the app (libpibolt.so). Feature
   request for https://github.com/opensec-git/Pi-Bolt.
2. **PC with adb/scrcpy (works today, no code)** — `adb shell screenrecord`
   or `scrcpy --record=file.mp4` on a computer, then Quick Share the file
   into `Download/` and analyze as above.
3. **Web-app demos** — for lacoquille UI demos specifically, an
   agent-browser/playwright CDP screencast on a PC works; on-device,
   preview_screenshot loops only.
