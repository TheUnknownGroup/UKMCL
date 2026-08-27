#!/bin/bash
# Update icon cache so the app icon shows up properly
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true
fi

# Update desktop database so the app shows up in menus/launchers
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications 2>/dev/null || true
fi

# Best-effort: launch the app for the user who ran the install, if they
# have an active graphical session. Never fails the install if this doesn't work.
if [ -n "$SUDO_USER" ]; then
    USER_ID=$(id -u "$SUDO_USER")
    XDG_RUNTIME_DIR="/run/user/$USER_ID"

    runuser -u "$SUDO_USER" -- frontend
fi

exit 0
