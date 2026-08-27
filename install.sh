#!/bin/sh
set -eu

user="$(id -un)"
home="$HOME"

sudo env \
    PACKAGE_USER="$user" \
    PACKAGE_HOME="$home" \
    dpkg -i ./*.deb

exit 0