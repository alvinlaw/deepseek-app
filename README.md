# DeepSeek Desktop

A lightweight, cross-platform desktop wrapper for the official [DeepSeek](https://chat.deepseek.com) web app, built with [Tauri 2](https://tauri.app/).

Runs the real DeepSeek web interface in a native window — no API key, no account lock-in, just a clean desktop app for Linux, Windows, and macOS.

## Features

- 🪟 Native window (not a browser tab) for `chat.deepseek.com`
- 🖥️ Cross-platform: Linux (.deb), Windows (.msi), macOS (.dmg)
- 🪶 Tiny bundle (~15 MB) thanks to Tauri's Rust + system webview
- 🔒 No telemetry, no middleman — talks directly to DeepSeek

## Download

Grab the latest release for your platform from the [Releases](https://github.com/alvinlaw/deepseek-app/releases) page:

| Platform | Package |
|----------|---------|
| Linux (Debian/Ubuntu) | `DeepSeek_x.y.z_amd64.deb` |
| Windows | `DeepSeek_x.y.z_x64_en-US.msi` |
| macOS (Intel) | `DeepSeek_x.y.z_x64.dmg` |
| macOS (Apple Silicon) | `DeepSeek_x.y.z_aarch64.dmg` |

## Build from source

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) 18+
- Platform webview deps:
  - **Linux**: `libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev libsoup-3.0-dev build-essential`
  - **Windows / macOS**: handled automatically by Tauri CLI

### Steps

```bash
git clone https://github.com/alvinlaw/deepseek-app
cd deepseek-app
npm install
npm run tauri build
```

Output binaries land in `src-tauri/target/release/bundle/`.

## Development

```bash
npm run tauri dev
```

## How it works

The app loads `https://chat.deepseek.com` inside a `WebView`. That's it — it's a faithful wrapper, not a reimplementation. Login, history, and features are all handled by DeepSeek's own service.

## Disclaimer

This is an **unofficial** community project and is not affiliated with, endorsed by, or sponsored by DeepSeek. Use at your own discretion and respect DeepSeek's [Terms of Service](https://chat.deepseek.com/terms).

## License

[MIT](LICENSE) © alvinlaw
