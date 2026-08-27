#!/bin/bash

getent passwd | awk -F: '$3 >= 1000 && $3 < 60000 {print $6}' | while read -r userhome; do
    [ -d "$userhome/.ukmcl" ] && rm -rf "$userhome/.ukmcl"
done
