#!/bin/bash
REAL_HOME=$(genent passwd "$SUDO_USER" | cut -d: -f6)
rm -rf $REAL_HOME/.ukmcl/
