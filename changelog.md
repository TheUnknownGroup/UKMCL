# v0.0.1
## What changed?
Updated instance list, added css and cards so every instance you make will have a nice looking card, along with it being on a grid.

Added the ability to delete instances, along with a custom popup box to double confirm deleting is what you want.

Added the function to create the main directory in the user's home directory. 

The home directories for each platform are as follows: 
* Windows is ``C:\Users\{your user}\.ukmcl\(subdirectories)``
* macOS is ``/Users/{your user}/.ukmcl/(subdirectories)``
* Linux is ``/home/{your user}/.ukmcl/(subdirectories)``

Updated the side panel to follow to the user as they're scrolling for easy of use and fast access to each page.

Changed the font to A Pompadour instead of Alegreya.

Added post install and post removal scripts as due to the app already creating the .ukmcl directory, say you'd uninstall the app if you so choose and don't want to have to scavenge everywhere to find it; the post removal script does that for you. 

If it doesn't then that means your OS doesn't support the command that's used to delete directories recursively. If thats the case, you're welcome to delete the directory manually by referencing [here](https://github.com/TheUnknownGroup/UKMCL/wiki#what-are-the-home-directories).

## What's expected to come next?
Currently working on the JVM portion to actually work with the adding of the instances, once its up it'll only be ready to run offline accounts along with no mods, but its a start!
