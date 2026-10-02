# v1.0.4
## What changed?
Updated the main config to support for a user set Offline / Microsoft account for Minecraft. Microsoft Auth is currently not supported but offline accounts are more than plausible.

Added the ability to change the memory usage globally, so each instance will run with the same amount of memory. (This make change.)

On the start of running the app, it creates the main config which--as said previously--can be edited in the settings page.

Added a Discord Rich presence, that will show on the start of the app, and accumulate the amount of time you've been on it.

Added a window for making an offline account.

Moved all of the commands in lib.rs inside of ``src-tauri/src`` to their own .rs files.

Slightly updated the download-bar css to move the percentage of total downloaded assets to be in the dead center of the bar.

When changing how much memory is being used, it'll show in the placeholder so you don't have to guess. It can be any number, but do not exceed half or 75% of the total ram you have.

Your account whether it be a Microsoft one or a Offline account, will show at the bottom of the instance's page and the main page.

## What's expected to come next?
Will add microsoft authentication for minecraft accounts and playing.

Updating the css of the instance window.