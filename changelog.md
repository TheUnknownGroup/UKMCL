# v2
## What changed?
Huge changes to the launcher.

Updated the entire UI by changing the window's transparency, added rounded edges, and finally added a dark & light mode.

Redid the entire ``/crates`` directory as it was cluttered and had far too many of them.

Switched to Typescript, and added Bootstrap icons to the mix.

Fastened the quickness in which a instance is made, also lowering the amount of memory it consumes on doing so. On first run with a clean install, I'm sure that'll be higher but its definitely a significant drop after creating your first instance.

You can now sign out of your account, and in the background the launcher will reload your account to ensure you're still able to play.

Better progress updates, rather than a finicky window that pops up every now and then when some asset's aren't installed but it was too late to close it quick it enough.

Added a general toast that will pop up on all windows -- will try to find a way for it to only pop up on the current focus --, displaying relative information, errors, and successes.

Removed svg files and switched them for their bootstrap icon's ``<i>`` element allowing for less lazy loading and decoding images. 

Added a LOT of css, some of which might be deleted later on, but for right now those classes and such will be temporary.

## What's expected to come next?
Forge installing. A revamp of the previous modpack download, mod download, and possibly mod managing.