#!/bin/bash
set -eu
# Update icon cache so the app icon shows up properly
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true
fi

# Update desktop database so the app shows up in menus/launchers
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications 2>/dev/null || true
fi 

user="${PACKAGE_USER:-}"
home="${PACKAGE_HOME:-}"

if [ -z "$user" ] || [ "$user" = root ]; then
    echo "No non-root user supplied; skipping user-specific setup." >&2
    exit 0
fi

if ! id "$user" >/dev/null 2>&1; then
    echo "Unknown user: $user" >&2
    exit 1
fi

runuser -u "$user" -- env \
    HOME="$home" \
    USER="$user" \
    LOGNAME="$user" \
    /usr/share/ukmcl/launch.sh

exit 0
