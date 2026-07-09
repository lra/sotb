# Shadow of the Blitz v 0.3.0

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

## Building

From the repository root:

```
cargo run
```

## Prebuilt binaries

When the `version` in `Cargo.toml` is bumped on `master`, CI publishes a [GitHub Release](https://github.com/lra/sotb/releases) with archives for:

- Linux x86_64 and ARM64
- macOS ARM64 and x86_64

Each archive is a single `sotb` binary — BMP assets under `data/` are embedded at compile time. Release binaries are still dynamically linked against system SDL3, so install SDL3 on the machine that runs them (same packages as above; runtime libs are enough if you only run the binary).

Release notes are generated automatically from commits and pull requests since the previous tag.

## Some useful keys

- F: Toggle Fullscreen
- ESC: Exit
