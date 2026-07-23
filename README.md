# Shadow of the Blitz v 0.4.0

Shadow of the Blitz has been rewritten in Rust, still on SDL (the `sdl3` crate).
It's as simple as it was, and I've only done it to see how SDL handle some parallax scrolls.

![Shadow of the Blitz parallax scroll](demo.gif)

## Requirements

- **Rust** with the 2024 edition (rustc 1.85+; `rustc --version` if unsure)
- **SDL3** system library (the crate links against it via pkg-config)

### Install SDL3

**macOS** (Homebrew):

```
brew install sdl3
```

**Linux** (package names vary by distro):

```
# Debian / Ubuntu (libsdl3-dev needs Ubuntu 25.04+ / Debian 13+; universe on Ubuntu)
sudo apt install libsdl3-dev pkg-config

# Fedora
sudo dnf install SDL3-devel pkgconf-pkg-config

# Arch
sudo pacman -S sdl3 pkgconf
```

## Building (native)

From the repository root:

```
cargo run
```

## Web / WASM

The same demo builds for the browser via [Emscripten](https://emscripten.org/) (`wasm32-unknown-emscripten`). SDL3 is compiled from source and linked statically; BMP assets stay embedded in the wasm module.

### Prerequisites

- **Rust** with the `wasm32-unknown-emscripten` target (`rustup target add wasm32-unknown-emscripten`)
- **Emscripten SDK** with `emcc` on your `PATH` (activate with `source ~/emsdk/emsdk_env.sh` after install)
- **CMake** (used when SDL3 is built from source for the web target)

Install emsdk (once):

```
git clone https://github.com/emscripten-core/emsdk.git ~/emsdk
cd ~/emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh   # or emsdk_env.fish
```

### Build and serve

```
source ~/emsdk/emsdk_env.sh   # if emcc is not already on PATH
./scripts/build-web.sh
python3 -m http.server -d web 8080
```

Open [http://127.0.0.1:8080/](http://127.0.0.1:8080/). Opening `web/index.html` as a `file://` URL will not work (browsers block WASM that way).

The script copies `sotb.js` and `sotb.wasm` into `web/` next to the checked-in `web/index.html`.

## Prebuilt binaries

When the `version` in `Cargo.toml` is bumped on `master`, CI publishes a [GitHub Release](https://github.com/lra/sotb/releases) with archives for:

- Linux x86_64 and ARM64
- macOS ARM64 and x86_64
- Web (`wasm32-unknown-emscripten`): `index.html`, `sotb.js`, `sotb.wasm` — extract onto any static HTTP host (or embed `sotb.js`/`sotb.wasm` in your own page) and it runs in the browser

Each native archive is a single `sotb` binary — BMP assets under `data/` are embedded at compile time. Release binaries are still dynamically linked against system SDL3, so install SDL3 on the machine that runs them (same packages as above; runtime libs are enough if you only run the binary).

Release notes are generated automatically from commits and pull requests since the previous tag.

## Some useful keys

- F: Toggle Fullscreen
- ESC: Exit
