#!/usr/bin/env bash
# Build sotb for the browser (wasm32-unknown-emscripten) and stage files under web/.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Prefer an already-activated emcc; otherwise try a common emsdk install path.
if ! command -v emcc >/dev/null 2>&1; then
  for envf in "${EMSDK}/emsdk_env.sh" "$HOME/emsdk/emsdk_env.sh"; do
    if [[ -f "$envf" ]]; then
      # shellcheck disable=SC1090
      source "$envf"
      break
    fi
  done
fi

if ! command -v emcc >/dev/null 2>&1; then
  cat >&2 <<'EOF'
emcc not found. Install and activate the Emscripten SDK, then re-run:

  git clone https://github.com/emscripten-core/emsdk.git ~/emsdk
  cd ~/emsdk
  ./emsdk install latest
  ./emsdk activate latest
  source ./emsdk_env.sh

See https://emscripten.org/docs/getting_started/downloads.html
EOF
  exit 1
fi

if ! command -v cmake >/dev/null 2>&1; then
  echo "cmake is required (SDL3 is built from source for Emscripten)." >&2
  exit 1
fi

rustup target add wasm32-unknown-emscripten >/dev/null

echo "Building release wasm (this compiles SDL3 on first run)…"
cargo build --release --target wasm32-unknown-emscripten --locked

OUT="target/wasm32-unknown-emscripten/release"
mkdir -p web

# Emscripten/cargo may emit sotb.js + sotb.wasm (and sometimes .data / .worker.js).
copied=0
for f in sotb.js sotb.wasm sotb.wasm.map sotb.worker.js; do
  if [[ -f "$OUT/$f" ]]; then
    cp -f "$OUT/$f" web/
    echo "  → web/$f"
    copied=1
  fi
done

# Some toolchains put the js next to a differently named wasm; copy any matching pair.
if [[ "$copied" -eq 0 ]]; then
  echo "No sotb.js/sotb.wasm under $OUT; listing:" >&2
  ls -la "$OUT" >&2 || true
  exit 1
fi

if [[ ! -f web/index.html ]]; then
  echo "web/index.html is missing." >&2
  exit 1
fi

cat <<EOF

Build complete. Serve over HTTP (file:// will not load WASM):

  python3 -m http.server -d web 8080

Then open http://127.0.0.1:8080/
EOF
