#!/usr/bin/env bash
# Fixes: "Could not create default EGL display: EGL_BAD_PARAMETER" on Wayland.
#
# Root cause (see https://github.com/tauri-apps/tauri/issues/15665):
# Tauri's AppImage bundler (linuxdeploy) sweeps libwayland-client.so.0 into
# usr/lib/. On newer host Mesa (25+) under a native Wayland session, WebKit's
# eglGetDisplay(EGL_DEFAULT_DISPLAY) fails when it resolves against that
# bundled, older libwayland-client instead of the host's — and
# WebKitWebProcess aborts before the window ever renders.
#
# Fix: make AppRun set LD_PRELOAD to the *host's* libwayland-client.so before
# launching the real binary, so the host lib wins regardless of what's
# bundled. Non-destructive — falls through untouched if no host lib is found.
#
# Usage: patch-appimage-wayland.sh <path-to-AppImage>

set -euo pipefail

APPIMAGE="${1:?Usage: patch-appimage-wayland.sh <path-to-AppImage>}"
WORKDIR="$(mktemp -d)"
trap 'rm -rf "$WORKDIR"' EXIT

echo "Patching $APPIMAGE for Wayland EGL compatibility..."

cp "$APPIMAGE" "$WORKDIR/app.AppImage"
chmod +x "$WORKDIR/app.AppImage"

pushd "$WORKDIR" > /dev/null
./app.AppImage --appimage-extract > /dev/null
popd > /dev/null

APPDIR="$WORKDIR/squashfs-root"

# The real entrypoint linuxdeploy generated becomes AppRun.real; our wrapper
# takes over as AppRun.
mv "$APPDIR/AppRun" "$APPDIR/AppRun.real"

cat > "$APPDIR/AppRun" << 'WRAPPER'
#!/usr/bin/env bash
set -e
HERE="$(dirname "$(readlink -f "${0}")")"

# Only intervene if the caller hasn't already forced an LD_PRELOAD.
if [ -z "${LD_PRELOAD:-}" ]; then
  for lib in \
    /usr/lib/x86_64-linux-gnu/libwayland-client.so.0 \
    /usr/lib64/libwayland-client.so.0 \
    /usr/lib/libwayland-client.so.0 \
    /usr/lib/aarch64-linux-gnu/libwayland-client.so.0 \
    /usr/lib/arm-linux-gnueabihf/libwayland-client.so.0; do
    if [ -f "$lib" ]; then
      export LD_PRELOAD="$lib"
      break
    fi
  done
fi

exec "$HERE/AppRun.real" "$@"
WRAPPER

chmod +x "$APPDIR/AppRun"

# Repackage. appimagetool must be on PATH (installed in the CI step below).
OUTPUT="$(basename "$APPIMAGE")"
rm -f "$WORKDIR/$OUTPUT"
ARCH=x86_64 appimagetool "$APPDIR" "$WORKDIR/$OUTPUT" > /dev/null

mv "$WORKDIR/$OUTPUT" "$APPIMAGE"
echo "Patched $APPIMAGE in place."
