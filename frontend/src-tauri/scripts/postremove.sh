#!/bin/bash
# 
if [ -z "$SUDO_USER" ]; then
     echo "This script must be run with sudo" >&2
     exit 1
fi
rm -rf "/home/$SUDO_USER/.ukmcl/"
