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
# Debian / Ubuntu
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

## Some useful keys

- F: Toggle Fullscreen
- ESC: Exit
